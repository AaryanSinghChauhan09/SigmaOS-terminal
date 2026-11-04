use anyhow::Result;
use sigmaos_component::{Config, Component, ComponentStatus, terminal};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_lifecycle_aaa() -> Result<()> {
        let config = Config::default();
        let mut term = terminal::new(config);
        
        assert_eq!(term.status(), ComponentStatus::Stopped);
        term.initialize()?;
        term.start()?;
        assert_eq!(term.status(), ComponentStatus::Running);
        term.stop()?;
        assert_eq!(term.status(), ComponentStatus::Stopped);
        Ok(())
    }

    #[test]
    fn test_terminal_metadata_aaa() {
        let config = Config::default();
        let term = terminal::new(config);
        assert_eq!(term.name(), "SigmaOS-terminal");
        assert_eq!(term.version(), "0.1.0");
    }
}
