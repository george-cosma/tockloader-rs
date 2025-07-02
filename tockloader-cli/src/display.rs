// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use tockloader_lib::attributes::app_attributes::AppAttributes;
use tockloader_lib::attributes::system_attributes::SystemAttributes;

// ANSI escape codes for colors
const RESET: &str = "\x1b[0m";
const BOLD_MAGENTA: &str = "\x1b[1;35m";
const BOLD_RED: &str = "\x1b[1;31m";
const BOLD_GREEN: &str = "\x1b[1;32m";
const BOLD_YELLOW: &str = "\x1b[1;33m";

pub async fn print_list(app_details: &[AppAttributes]) {
    for (i, details) in app_details.iter().enumerate() {
        println!("\n{RESET}{BOLD_MAGENTA} ┏━━━━━━━━━━━━━━━━┓");
        println!("{RESET}{BOLD_RED} ┃ {RESET}{BOLD_GREEN} App_{i} {RESET}{BOLD_RED}┃",);
        println!("{RESET}{BOLD_YELLOW} ┗━━━━━━━━━━━━━━━━┛");
        println!(
            "\n {BOLD_GREEN} Name:             {RESET}{}",
            details.tbf_header.get_package_name().unwrap(),
        );

        println!(
            " {BOLD_GREEN} Version:          {RESET}{}",
            details.tbf_header.get_binary_version(),
        );

        println!(
            " {BOLD_GREEN} Enabled:          {RESET}{}",
            details.tbf_header.enabled(),
        );

        println!(
            " {BOLD_GREEN} Sticky:           {RESET}{}",
            details.tbf_header.sticky(),
        );

        println!(
            " {BOLD_GREEN} Total_Size:       {RESET}{}\n\n",
            details.tbf_header.total_size(),
        );
    }
}

pub async fn print_info(app_details: &mut [AppAttributes], system_details: &mut SystemAttributes) {
    for (i, details) in app_details.iter().enumerate() {
        println!("\n{RESET}{BOLD_MAGENTA} ┏━━━━━━━━━━━━━━━━┓");
        println!("{RESET}{BOLD_RED} ┃ {RESET}{BOLD_GREEN} App_{i} {RESET}{BOLD_RED}┃");
        println!("{RESET}{BOLD_YELLOW} ┗━━━━━━━━━━━━━━━━┛");

        println!(
            "\n {BOLD_GREEN} Name:             {RESET}{}",
            details.tbf_header.get_package_name().unwrap(),
        );

        println!(
            " {BOLD_GREEN} Version:          {RESET}{}",
            details.tbf_header.get_binary_version(),
        );

        println!(
            " {BOLD_GREEN} Enabled:          {RESET}{}",
            details.tbf_header.enabled(),
        );

        println!(
            " {BOLD_GREEN} Sticky:           {RESET}{}",
            details.tbf_header.sticky(),
        );

        println!(
            " {BOLD_GREEN} Total_Size:       {RESET}{}",
            details.tbf_header.total_size(),
        );

        println!(
            " {BOLD_GREEN} Address in Flash: {RESET}{}",
            system_details.appaddr.unwrap(),
        );

        println!(
            " {BOLD_GREEN}    TBF version:   {RESET}{}",
            details.tbf_header.get_binary_version(),
        );

        println!(
            " {BOLD_GREEN}    header_size:   {RESET}{}",
            details.tbf_header.header_size(),
        );

        println!(
            " {BOLD_GREEN}    total_size:    {RESET}{}",
            details.tbf_header.total_size(),
        );

        println!(
            " {BOLD_GREEN}    checksum:      {RESET}{}",
            details.tbf_header.checksum(),
        );

        println!(" {BOLD_GREEN}    flags:{RESET}");
        println!(
            " {BOLD_GREEN}        enabled:       {RESET}{}",
            details.tbf_header.enabled(),
        );

        println!(
            " {BOLD_GREEN}        sticky:        {RESET}{}",
            details.tbf_header.sticky(),
        );

        println!(" {BOLD_GREEN}    TVL: Main (1){RESET}");
        println!(
            " {BOLD_GREEN}        init_fn_offset:        {RESET}{}",
            details.tbf_header.get_init_function_offset(),
        );

        println!(
            " {BOLD_GREEN}        protected_size:        {RESET}{}",
            details.tbf_header.get_protected_size(),
        );

        println!(
            " {BOLD_GREEN}        minimum_ram_size:      {RESET}{}",
            details.tbf_header.get_minimum_app_ram_size(),
        );

        println!(" {BOLD_GREEN}    TVL: Program (9){RESET}");
        println!(
            " {BOLD_GREEN}        init_fn_offset:        {RESET}{}",
            details.tbf_header.get_init_function_offset(),
        );

        println!(
            " {BOLD_GREEN}        protected_size:        {RESET}{}",
            details.tbf_header.get_protected_size(),
        );

        println!(
            " {BOLD_GREEN}        minimum_ram_size:      {RESET}{}",
            details.tbf_header.get_minimum_app_ram_size(),
        );

        println!(
            " {BOLD_GREEN}        binary_end_offset:     {RESET}{}",
            details.tbf_header.get_binary_end(),
        );

        println!(
            " {BOLD_GREEN}        app_version:           {RESET}{}",
            details.tbf_header.get_binary_version(),
        );

        println!(" {BOLD_GREEN}    TVL: Package Name (3){RESET}");
        println!(
            " {BOLD_GREEN}        package_name:          {RESET}{}",
            details.tbf_header.get_package_name().unwrap(),
        );

        println!(" {BOLD_GREEN}    TVL: Kernel Version (8){RESET}");
        println!(
            " {BOLD_GREEN}        kernel_major:          {RESET}{}",
            details.tbf_header.get_kernel_version().unwrap().0,
        );

        println!(
            " {BOLD_GREEN}        kernel_minor:          {RESET}{}",
            details.tbf_header.get_kernel_version().unwrap().1,
        );

        println!("\n {BOLD_GREEN}    Footer{RESET}");

        let mut total_footer_size: u32 = 0;

        // Usage of +4 is a result of the structure of the Tock Binary Format (https://book.tockos.org/doc/tock_binary_format)
        // Because we need the real size of the footer including the type and length.
        for footer_details in details.tbf_footers.iter() {
            total_footer_size += footer_details.size + 4;
        }

        println!(" {BOLD_GREEN}            footer_size:       {RESET}{total_footer_size}");

        for (j, footer_details) in details.tbf_footers.iter().enumerate() {
            println!(" {BOLD_GREEN}    Footer [{j}] TVL: Credentials{RESET}");

            println!(
                " {BOLD_GREEN}        Type:                  {RESET}{}",
                footer_details.credentials.get_type(),
            );

            // Usage of -4 is a result of the structure of the Tock Binary Format (https://book.tockos.org/doc/tock_binary_format)
            // Because we only need the size of the credentials without the type and length bytes.
            println!(
                " {BOLD_GREEN}        Length:                {RESET}{}",
                footer_details.size - 4,
            );
        }
    }

    println!("\n\n{BOLD_GREEN} Kernel Attributes{RESET}");
    println!(
        "{BOLD_GREEN}    Sentinel:          {:<10}{RESET}",
        system_details.sentinel.clone().unwrap(),
    );
    println!(
        "{BOLD_GREEN}    Version:           {:<10}{RESET}",
        system_details.kernel_version.unwrap(),
    );
    println!("{BOLD_GREEN} KATLV: APP Memory{RESET}");
    println!(
        "{BOLD_GREEN}    app_memory_start:  {:<10}{RESET}",
        system_details.app_mem_start.unwrap(),
    );
    println!(
        "{BOLD_GREEN}    app_memory_len:    {:<10}{RESET}",
        system_details.app_mem_len.unwrap(),
    );
    println!("{BOLD_GREEN} KATLV: Kernel Binary{RESET}");
    println!(
        "{BOLD_GREEN}    kernel_binary_start: {:<10}{RESET}",
        system_details.kernel_bin_start.unwrap(),
    );
    println!(
        "{BOLD_GREEN}    kernel_binary_len:   {:<10}{RESET}\n\n",
        system_details.kernel_bin_len.unwrap(),
    );
}
