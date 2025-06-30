use async_trait::async_trait;

use crate::board_settings::BoardSettings;
use crate::connection::SerialConnection;
use crate::errors::TockloaderError;
use crate::tabs::tab::Tab;
use crate::CommandInstall;

#[async_trait]
impl CommandInstall for SerialConnection {
    async fn install_app(
        &mut self,
        _settings: &BoardSettings,
        _tab_file: Tab,
    ) -> Result<(), TockloaderError> {
        todo!()
    }
}

// pub async fn install_app(
//     choice: Connection,
//     core_index: Option<&usize>,
//     tab_file: Tab,
// ) -> Result<(), TockloaderError> {
//     match choice {
//         Connection::ProbeRS(mut session) => {
//             // *snip*
//         }
//         Connection::Serial(mut port) => {
//             let response = ping_bootloader_and_wait_for_response(&mut port).await?;
//
//             if response as u8 != Response::Pong as u8 {
//                 tokio::time::sleep(Duration::from_millis(100)).await;
//                 let _ = ping_bootloader_and_wait_for_response(&mut port).await?;
//             }

//             let system_attributes =
//                 SystemAttributes::read_system_attributes_serial(&mut port).await?;

//             let board = system_attributes
//                 .board
//                 .ok_or("No board name found.".to_owned());
//             let kernel_version = system_attributes
//                 .kernel_version
//                 .ok_or("No kernel version found.".to_owned());

//             match board {
//                 Ok(board) => {
//                     // Verify if the specified app is compatible with board
//                     // TODO(Micu Ana): Replace the print with log messages
//                     if tab_file.is_compatible_with_board(&board) {
//                         println!("Specified tab is compatible with board.");
//                     } else {
//                         panic!("Specified tab is not compatible with board.");
//                     }
//                 }
//                 Err(e) => {
//                     return Err(TockloaderError::MisconfiguredBoard(e));
//                 }
//             }

//             match kernel_version {
//                 Ok(kernel_version) => {
//                     // Verify if the specified app is compatible with kernel version
//                     // TODO(Micu Ana): Replace the prints with log messages
//                     if tab_file.is_compatible_with_kernel_verison(kernel_version as u32) {
//                         println!("Specified tab is compatible with your kernel version.");
//                     } else {
//                         println!("Specified tab is not compatible with your kernel version.");
//                     }
//                 }
//                 Err(e) => {
//                     return Err(TockloaderError::MisconfiguredBoard(e));
//                 }
//             }

//             let mut address =
//                 system_attributes
//                     .appaddr
//                     .ok_or(TockloaderError::MisconfiguredBoard(
//                         "No start address found.".to_owned(),
//                     ))?;
//             loop {
//                 // Read a block of 200 8-bit words
//                 let mut pkt = (address as u32).to_le_bytes().to_vec();
//                 let length = (200_u16).to_le_bytes().to_vec();
//                 for i in length {
//                     pkt.push(i);
//                 }

//                 let (_, message) = issue_command(
//                     &mut port,
//                     Command::ReadRange,
//                     pkt,
//                     true,
//                     200,
//                     Response::ReadRange,
//                 )
//                 .await?;

//                 let (_ver, _header_len, whole_len) = match parse_tbf_header_lengths(
//                     &message[0..8]
//                         .try_into()
//                         .expect("Buffer length must be at least 8 bytes long."),
//                 ) {
//                     Ok((ver, header_len, whole_len)) if header_len != 0 => {
//                         (ver, header_len, whole_len)
//                     }
//                     _ => break, // No more apps
//                 };

//                 address += whole_len as u64;
//             }

//             let arch = system_attributes
//                 .arch
//                 .ok_or("No architecture found.".to_owned());

//             match arch {
//                 Ok(arch) => {
//                     let binary = tab_file.extract_binary(&arch.clone());

//                     match binary {
//                         Ok(mut binary) => {
//                             let size = binary.len() as u64;

//                             let multiple = address / size;

//                             let (mut new_address, _gap_size) = if multiple * size != address {
//                                 let new_address = ((address + size) / size) * size;
//                                 let gap_size = new_address - address;
//                                 (new_address, gap_size)
//                             } else {
//                                 (address, 0)
//                             };

//                             // Make sure the binary is a multiple of the page size by padding 0xFFs
//                             // TODO(Micu Ana): check if the page-size differs
//                             let page_size = 512;
//                             let needs_padding = binary.len() % page_size != 0;

//                             if needs_padding {
//                                 let remaining = page_size - (binary.len() % page_size);
//                                 for _i in 0..remaining {
//                                     binary.push(0xFF);
//                                 }
//                             }

//                             let binary_len = binary.len();

//                             // Get indices of pages that have valid data to write
//                             let mut valid_pages: Vec<u8> = Vec::new();
//                             for i in 0..(binary_len / page_size) {
//                                 for b in binary[(i * page_size)..((i + 1) * page_size)]
//                                     .iter()
//                                     .copied()
//                                 {
//                                     if b != 0 {
//                                         valid_pages.push(i as u8);
//                                         break;
//                                     }
//                                 }
//                             }

//                             // If there are no pages valid, all pages would have been removed, so we write them all
//                             if valid_pages.is_empty() {
//                                 for i in 0..(binary_len / page_size) {
//                                     valid_pages.push(i as u8);
//                                 }
//                             }

//                             // Include a blank page (if exists) after the end of a valid page. There might be a usable 0 on the next page
//                             let mut ending_pages: Vec<u8> = Vec::new();
//                             for &i in &valid_pages {
//                                 let mut iter = valid_pages.iter();
//                                 if !iter.any(|&x| x == (i + 1))
//                                     && (i + 1) < (binary_len / page_size) as u8
//                                 {
//                                     ending_pages.push(i + 1);
//                                 }
//                             }

//                             for i in ending_pages {
//                                 valid_pages.push(i);
//                             }

//                             for i in valid_pages {
//                                 // Create the packet that we send to the bootloader
//                                 // First four bytes are the address of the page
//                                 let mut pkt = (new_address as u32
//                                     + (i as usize * page_size) as u32)
//                                     .to_le_bytes()
//                                     .to_vec();
//                                 // Then the bytes that go into the page
//                                 for b in binary
//                                     [(i as usize * page_size)..((i + 1) as usize * page_size)]
//                                     .iter()
//                                     .copied()
//                                 {
//                                     pkt.push(b);
//                                 }

//                                 // Write to bootloader
//                                 let (_, _) = issue_command(
//                                     &mut port,
//                                     Command::WritePage,
//                                     pkt,
//                                     true,
//                                     0,
//                                     Response::OK,
//                                 )
//                                 .await?;
//                             }

//                             new_address += binary.len() as u64;

//                             let pkt = (new_address as u32).to_le_bytes().to_vec();

//                             let _ = issue_command(
//                                 &mut port,
//                                 Command::ErasePage,
//                                 pkt,
//                                 true,
//                                 0,
//                                 Response::OK,
//                             )
//                             .await?;
//                         }
//                         Err(e) => {
//                             return Err(e);
//                         }
//                     }
//                     Ok(())
//                 }
//                 Err(e) => Err(TockloaderError::MisconfiguredBoard(e)),
//             }
//         }
//     }
// }
