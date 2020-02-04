use super::args::PbExport;
use packybara::io::packages_xml::xml;
use packybara::packrat::Client;

/// Given a client and PbExport enum, extract the parameters from the enum and
/// export the provided show's state to disk
///
/// # Arguments
/// * `client` - a Client instanec
/// * `cmd` - a PbExport instance
///
/// # Returns
/// * A Unit if Ok, or a boxed error if Err
pub fn export(client: Client, cmd: PbExport) -> Result<(), Box<dyn std::error::Error>> {
    let PbExport::PackagesXml { show, path, .. } = cmd;
    let result = xml::write_xml(client, show, path)?;
    Ok(result)
}
