use super::args::PbExport;
use packybara::io::packages_xml::xml;
use packybara::packrat::Client;

pub fn export(client: Client, cmd: PbExport) -> Result<(), Box<dyn std::error::Error>> {
    let PbExport::PackagesXml { show, path, .. } = cmd;
    let result = xml::write_xml(client, show, path)?;
    Ok(result)
}
