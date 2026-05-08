use schemars::JsonSchema;
use serde::Deserialize;

use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const MCP_REMOTE_PACKAGE: &str = "mcp-remote";
const MCP_REMOTE_VERSION: &str = "0.1.29";
const SERVER_ID: &str = "zed-mcp-server-zai-web-search";
const MCP_URL: &str = "https://api.z.ai/api/mcp/web_search_prime/mcp";

struct ZaiWebSearchExtension;

#[derive(Debug, Deserialize, JsonSchema)]
struct ZaiWebSearchSettings {
    #[serde(default)]
    zai_api_key: Option<String>,
}

impl zed::Extension for ZaiWebSearchExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let version = zed::npm_package_installed_version(MCP_REMOTE_PACKAGE)?;
        if version.is_none() {
            zed::npm_install_package(MCP_REMOTE_PACKAGE, MCP_REMOTE_VERSION)?;
        }

        let settings = ContextServerSettings::for_project(SERVER_ID, project)?;
        let settings: ZaiWebSearchSettings = if let Some(settings_value) = settings.settings {
            serde_json::from_value(settings_value).map_err(|e| e.to_string())?
        } else {
            ZaiWebSearchSettings { zai_api_key: None }
        };

        let mut args = vec![MCP_URL.to_string()];
        if let Some(api_key) = settings.zai_api_key {
            args.push("--header".to_string());
            args.push(format!("Authorization: Bearer {}", api_key));
        }

        let command = if cfg!(target_os = "windows") {
            "node_modules/.bin/mcp-remote.cmd".to_string()
        } else {
            let path = "node_modules/.bin/mcp-remote";
            zed::make_file_executable(path)?;
            path.to_string()
        };

        Ok(Command {
            command,
            args,
            env: Vec::new(),
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();

        let mut default_settings =
            include_str!("../configuration/default_settings.jsonc").to_string();

        let settings = ContextServerSettings::for_project(SERVER_ID, project);
        if let Ok(user_settings) = settings {
            if let Some(settings_value) = user_settings.settings {
                if let Ok(my_settings) =
                    serde_json::from_value::<ZaiWebSearchSettings>(settings_value)
                {
                    match my_settings.zai_api_key {
                        Some(api_key) => {
                            let serialized_api_key =
                                serde_json::to_string(&api_key).map_err(|e| e.to_string())?;
                            default_settings = default_settings
                                .replace("\"YOUR_ZAI_API_KEY\"", &serialized_api_key);
                        }
                        None => {
                            default_settings =
                                default_settings.replace("\"YOUR_ZAI_API_KEY\"", "\"\"");
                        }
                    }
                }
            }
        }

        let settings_schema = serde_json::to_string(&schemars::schema_for!(ZaiWebSearchSettings))
            .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(ZaiWebSearchExtension);
