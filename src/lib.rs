use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Provider {
    Gmail,
    M365Exchange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MailProtocol {
    Imap,
    Smtp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    Linux,
    Windows,
    MacOs,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountTemplate {
    pub display_name: String,
    pub provider: Provider,
    pub color_hex: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityTemplate {
    pub oauth_enabled: bool,
    pub windows_defender_integration: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopIntegrationTemplate {
    pub system_notifications: bool,
    pub tray_icon: bool,
    pub startup_linux: bool,
    pub startup_windows: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarEvent {
    pub account_name: String,
    pub title: String,
    pub start_minute: u32,
    pub end_minute: u32,
}

impl CalendarEvent {
    pub fn overlaps(&self, other: &Self) -> bool {
        self.start_minute < other.end_minute && other.start_minute < self.end_minute
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CalendarBoard {
    pub events: Vec<CalendarEvent>,
}

impl CalendarBoard {
    pub fn overlapping_pairs(&self) -> Vec<(usize, usize)> {
        let mut pairs = Vec::new();

        for (left_index, left_event) in self.events.iter().enumerate() {
            for (right_index, right_event) in self.events.iter().enumerate().skip(left_index + 1) {
                if left_event.overlaps(right_event) {
                    pairs.push((left_index, right_index));
                }
            }
        }

        pairs
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailClientTemplate {
    pub accounts: Vec<AccountTemplate>,
    pub supported_platforms: HashSet<Platform>,
    pub security: SecurityTemplate,
    pub desktop_integration: DesktopIntegrationTemplate,
}

impl EmailClientTemplate {
    pub fn starter() -> Self {
        Self {
            accounts: vec![
                AccountTemplate {
                    display_name: "Gmail account".to_string(),
                    provider: Provider::Gmail,
                    color_hex: "#1A73E8".to_string(),
                },
                AccountTemplate {
                    display_name: "M365 account".to_string(),
                    provider: Provider::M365Exchange,
                    color_hex: "#0078D4".to_string(),
                },
            ],
            supported_platforms: HashSet::from([
                Platform::Linux,
                Platform::Windows,
                Platform::MacOs,
            ]),
            security: SecurityTemplate {
                oauth_enabled: true,
                windows_defender_integration: true,
            },
            desktop_integration: DesktopIntegrationTemplate {
                system_notifications: true,
                tray_icon: true,
                startup_linux: true,
                startup_windows: true,
            },
        }
    }

    pub fn protocols_for(provider: Provider) -> &'static [MailProtocol] {
        match provider {
            Provider::Gmail | Provider::M365Exchange => &[MailProtocol::Imap, MailProtocol::Smtp],
        }
    }

    pub fn validate(&self) -> Result<(), TemplateError> {
        if !self.supported_platforms.contains(&Platform::Linux) {
            return Err(TemplateError::MissingLinuxSupport);
        }

        if !self.security.oauth_enabled {
            return Err(TemplateError::MissingOAuth);
        }

        if !self.security.windows_defender_integration {
            return Err(TemplateError::MissingWindowsDefenderIntegration);
        }

        if !(self.desktop_integration.system_notifications
            && self.desktop_integration.tray_icon
            && self.desktop_integration.startup_linux
            && self.desktop_integration.startup_windows)
        {
            return Err(TemplateError::MissingDesktopIntegrationFeatures);
        }

        let mut colors = HashSet::new();
        for account in &self.accounts {
            if !is_hex_color(&account.color_hex) {
                return Err(TemplateError::InvalidAccountColor(
                    account.color_hex.clone(),
                ));
            }

            if !colors.insert(account.color_hex.as_str()) {
                return Err(TemplateError::DuplicateAccountColor(
                    account.color_hex.clone(),
                ));
            }
        }

        let providers = self
            .accounts
            .iter()
            .map(|account| account.provider)
            .collect::<HashSet<_>>();

        if !providers.contains(&Provider::Gmail) || !providers.contains(&Provider::M365Exchange) {
            return Err(TemplateError::MissingMultiProviderSetup);
        }

        Ok(())
    }
}

fn is_hex_color(value: &str) -> bool {
    value.len() == 7
        && value.starts_with('#')
        && value
            .as_bytes()
            .iter()
            .skip(1)
            .all(|byte| byte.is_ascii_hexdigit())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateError {
    MissingLinuxSupport,
    MissingOAuth,
    MissingWindowsDefenderIntegration,
    MissingDesktopIntegrationFeatures,
    DuplicateAccountColor(String),
    InvalidAccountColor(String),
    MissingMultiProviderSetup,
}

impl std::fmt::Display for TemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingLinuxSupport => write!(f, "linux support is required"),
            Self::MissingOAuth => write!(f, "OAuth must be enabled"),
            Self::MissingWindowsDefenderIntegration => {
                write!(f, "Windows Defender integration must be configured")
            }
            Self::MissingDesktopIntegrationFeatures => {
                write!(
                    f,
                    "desktop notifications, startup and tray support are required"
                )
            }
            Self::DuplicateAccountColor(color) => {
                write!(f, "account color must be unique: {color}")
            }
            Self::InvalidAccountColor(color) => write!(f, "invalid account color format: {color}"),
            Self::MissingMultiProviderSetup => {
                write!(
                    f,
                    "template must include both Gmail and M365 account providers"
                )
            }
        }
    }
}

impl std::error::Error for TemplateError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starter_template_meets_requirements() {
        let template = EmailClientTemplate::starter();

        assert!(template.validate().is_ok());
    }

    #[test]
    fn duplicate_colors_are_rejected() {
        let mut template = EmailClientTemplate::starter();
        template.accounts[1].color_hex = template.accounts[0].color_hex.clone();

        assert_eq!(
            template.validate(),
            Err(TemplateError::DuplicateAccountColor("#1A73E8".to_string()))
        );
    }

    #[test]
    fn calendar_overlap_detection_supports_overlapping_agendas() {
        let board = CalendarBoard {
            events: vec![
                CalendarEvent {
                    account_name: "Gmail account".to_string(),
                    title: "Daily standup".to_string(),
                    start_minute: 540,
                    end_minute: 570,
                },
                CalendarEvent {
                    account_name: "M365 account".to_string(),
                    title: "Client sync".to_string(),
                    start_minute: 560,
                    end_minute: 620,
                },
                CalendarEvent {
                    account_name: "M365 account".to_string(),
                    title: "Focus block".to_string(),
                    start_minute: 700,
                    end_minute: 730,
                },
            ],
        };

        assert_eq!(board.overlapping_pairs(), vec![(0, 1)]);
    }

    #[test]
    fn providers_expose_imap_and_smtp_protocols() {
        for provider in [Provider::Gmail, Provider::M365Exchange] {
            let protocols = EmailClientTemplate::protocols_for(provider);
            assert!(protocols.contains(&MailProtocol::Imap));
            assert!(protocols.contains(&MailProtocol::Smtp));
        }
    }
}
