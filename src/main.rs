use windows::core::*;
use windows::Management::Deployment::*;

fn main() -> Result<()> {
    // 1. Get https://github.com/AppInstaller/StreamingInstallDemoApp, compile and deploy onto machine
    // 2. Compile crate, run elevated

    let package_manager = PackageManager::new()?;
    let package = package_manager
        .FindPackagesByPackageFamilyName("96c3ec89-c3ad-4c5f-9ab9-b225c9e2f946_p2vdx7b0qnd28")?
        .First()?
        .Current()?;

    // Fails with Error: Error { code: 0x00000000, message: The operation completed successfully. }
    // Doc: Returns Null if the named content group is not part of this package.
    // https://docs.microsoft.com/uwp/api/windows.applicationmodel.package.getcontentgroupasync
    let group = package
        .GetContentGroupAsync("NonExistentContentGroup")?
        .get()?;

    // Works
    // let group = package.GetContentGroupAsync("ItemPack1")?.get()?;

    println!("Name: {}, State: {:?}", group.Name()?, group.State()?);
    Ok(())
}
