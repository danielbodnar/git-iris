//! Project documentation tool for Rig-based agents
//!
//! This tool fetches documentation files like README.md, CONTRIBUTING.md,
//! CHANGELOG.md, etc. from the project root.

use anyhow::Result;
use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::common::{current_repo_root, parameters_schema};

// Use standard tool error macro for consistency
crate::define_tool_error!(DocsError);

const MAX_DOC_CHARS: usize = 20_000;
const MAX_CONTEXT_TOTAL_CHARS: usize = 8_000;
const README_CONTEXT_CHARS: usize = 4_000;
const AGENT_CONTEXT_CHARS: usize = 4_000;
const OTHER_CONTEXT_CHARS: usize = 2_000;

/// Tool for fetching project documentation files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDocs;

/// Type of documentation to fetch
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema, Default)]
#[serde(rename_all = "lowercase")]
pub enum DocType {
    /// README file (README.md, README.rst, README.txt)
    #[default]
    Readme,
    /// Contributing guidelines (CONTRIBUTING.md)
    Contributing,
    /// Changelog (CHANGELOG.md, HISTORY.md)
    Changelog,
    /// License file (LICENSE, LICENSE.md)
    License,
    /// Code of conduct (`CODE_OF_CONDUCT.md`)
    CodeOfConduct,
    /// Agent/AI instructions (AGENTS.md, CLAUDE.md, .github/copilot-instructions.md)
    Agents,
    /// Project context: README + agent instructions (recommended for all operations)
    Context,
    /// All documentation files
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ProjectDocsArgs {
    /// Type of documentation to fetch
    #[serde(default)]
    pub doc_type: DocType,
    /// Maximum characters to return (default: 20000, max: 20000).
    /// For `context`, this is a total shared budget across the returned snapshot.
    #[serde(default = "default_max_chars")]
    pub max_chars: usize,
}

fn default_max_chars() -> usize {
    MAX_DOC_CHARS
}

fn context_doc_budget(filename: &str, remaining: usize) -> usize {
    let preferred_budget = if filename.eq_ignore_ascii_case("README.md")
        || filename.eq_ignore_ascii_case("README.rst")
        || filename.eq_ignore_ascii_case("README.txt")
        || filename.eq_ignore_ascii_case("README")
        || filename.eq_ignore_ascii_case("readme.md")
    {
        README_CONTEXT_CHARS
    } else if matches!(
        filename,
        "AGENTS.md" | "CLAUDE.md" | ".github/copilot-instructions.md" | "CODING_GUIDELINES.md"
    ) || filename.starts_with(".cursor/")
    {
        AGENT_CONTEXT_CHARS
    } else {
        OTHER_CONTEXT_CHARS
    };

    preferred_budget.min(remaining)
}

fn append_doc(
    output: &mut String,
    filename: &str,
    content: &str,
    max_chars: usize,
    truncated_hint: Option<&str>,
) {
    output.push_str(&format!("=== {} ===\n", filename));

    let char_count = content.chars().count();
    if char_count > max_chars {
        let truncated: String = content.chars().take(max_chars).collect();
        output.push_str(&truncated);

        if let Some(hint) = truncated_hint {
            output.push_str(&format!(
                "\n\n[... context snapshot truncated after {} chars; {} ...]\n",
                max_chars, hint
            ));
        } else {
            output.push_str(&format!(
                "\n\n[... truncated, {} more chars ...]\n",
                char_count - max_chars
            ));
        }
    } else {
        output.push_str(content);
    }

    output.push_str("\n\n");
}

impl Tool for ProjectDocs {
    const NAME: &'static str = "project_docs";
    type Error = DocsError;
    type Args = ProjectDocsArgs;
    type Output = String;

    async fn definition(&self, _: String) -> ToolDefinition {
        ToolDefinition {
            name: "project_docs".to_string(),
            description:
                "Fetch project documentation for context. Types: readme, contributing, changelog, license, codeofconduct, agents (AGENTS.md/CLAUDE.md), context (compact README + agent-instructions snapshot), all"
                    .to_string(),
            parameters: parameters_schema::<ProjectDocsArgs>(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let current_dir = current_repo_root().map_err(DocsError::from)?;
        let max_chars = args.max_chars.min(MAX_DOC_CHARS);
        let is_context = matches!(args.doc_type, DocType::Context);

        let files_to_check = match args.doc_type {
            DocType::Readme => vec![
                "README.md",
                "README.rst",
                "README.txt",
                "README",
                "readme.md",
            ],
            DocType::Contributing => vec!["CONTRIBUTING.md", "CONTRIBUTING", "contributing.md"],
            DocType::Changelog => vec![
                "CHANGELOG.md",
                "CHANGELOG",
                "HISTORY.md",
                "CHANGES.md",
                "changelog.md",
            ],
            DocType::License => vec!["LICENSE", "LICENSE.md", "LICENSE.txt", "license"],
            DocType::CodeOfConduct => vec!["CODE_OF_CONDUCT.md", "code_of_conduct.md"],
            DocType::Agents => vec![
                "AGENTS.md",
                "CLAUDE.md",
                ".github/copilot-instructions.md",
                ".cursor/rules",
                "CODING_GUIDELINES.md",
            ],
            DocType::Context => vec![
                "README.md",
                "AGENTS.md",
                "CLAUDE.md",
                ".github/copilot-instructions.md",
            ],
            DocType::All => vec![
                "README.md",
                "AGENTS.md",
                "CLAUDE.md",
                "CONTRIBUTING.md",
                "CHANGELOG.md",
                "CODE_OF_CONDUCT.md",
            ],
        };

        let mut output = String::new();
        let mut found_any = false;
        let mut remaining_context_chars = max_chars.min(MAX_CONTEXT_TOTAL_CHARS);
        // Track if we found an agent instructions file (AGENTS.md often symlinks to CLAUDE.md)
        let mut found_agent_doc = false;

        if is_context {
            output.push_str(
                "Project context snapshot. This is intentionally compact so you can get conventions quickly.\n",
            );
            output.push_str(
                "Use `project_docs(doc_type=\"readme\")` or `project_docs(doc_type=\"agents\")` if you need the full documents.\n\n",
            );
        }

        for filename in files_to_check {
            // Skip CLAUDE.md if we already found AGENTS.md (avoid duplicate from symlink)
            if filename == "CLAUDE.md" && found_agent_doc {
                continue;
            }

            if is_context && remaining_context_chars == 0 {
                break;
            }

            let path: PathBuf = current_dir.join(filename);
            if path.exists() {
                match tokio::fs::read_to_string(&path).await {
                    Ok(content) => {
                        found_any = true;

                        // Mark that we found an agent doc file
                        if filename == "AGENTS.md" {
                            found_agent_doc = true;
                        }

                        if is_context {
                            let doc_budget = context_doc_budget(filename, remaining_context_chars);
                            append_doc(
                                &mut output,
                                filename,
                                &content,
                                doc_budget,
                                Some("call the targeted doc type for the full file"),
                            );
                            remaining_context_chars =
                                remaining_context_chars.saturating_sub(doc_budget);
                        } else {
                            append_doc(&mut output, filename, &content, max_chars, None);
                        }

                        // For single doc types, return after finding first match
                        // Context and All gather multiple files
                        if !matches!(args.doc_type, DocType::All | DocType::Context) {
                            break;
                        }
                    }
                    Err(e) => {
                        output.push_str(&format!("Error reading {}: {}\n", filename, e));
                    }
                }
            }
        }

        if !found_any {
            output = format!(
                "No {:?} documentation found in project root.",
                args.doc_type
            );
        }

        Ok(output)
    }
}
