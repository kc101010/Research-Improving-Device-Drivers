use bindings::{
    Windows::Foundation::Uri,
    Windows::Web::Syndication::SyndicationClient,
    Windows::Win32::WindowsAndMessaging::{MessageBoxA, MESSAGEBOX_STYLE}    
};

//-> indicates return type
fn main() -> windows::Result<()> {
    let uri = Uri::CreateUri("https://blogs.windows.com/feed")?; //? for easier error handling
    let client = SyndicationClient::new()?;
    let feed = client.RetrieveFeedAsync(uri)?.get()?;

    for item in feed.Items()? {
        println!("{}", item.Title()?.Text()?);
    }

    for item in feed.Items()?{

        let mut txt = item.Title()?.Text()?;
        unsafe{
            MessageBoxA(None, "{txt}", "RSS Headers", MESSAGEBOX_STYLE::MB_OK);
        }
    }
   

    Ok(())
}
