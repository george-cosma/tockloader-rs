// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use tockloader_lib::attributes::{
    app_attributes::AppAttributes, system_attributes::SystemAttributes,
};

pub async fn print_list(app_details: &mut [AppAttributes]) {
    for (i, details) in app_details.iter().enumerate() {
        println!("\n\x1b[0m\x1b[1;35m ┏━━━━━━━━━━━━━━━━┓");
        println!(
            "\x1b[0m\x1b[1;31m ┃ \x1b[0m\x1b[1;32m App_{:<9?} \x1b[0m\x1b[1;31m┃",
            i
        );
        println!("\x1b[0m\x1b[1;33m ┗━━━━━━━━━━━━━━━━┛");
        println!(
            "\n \x1b[1;32m Name:                {}",
            details.tbf_header.get_package_name().unwrap()
        );

        println!(
            " \x1b[1;32m Version:             {}",
            details.tbf_header.get_binary_version()
        );

        println!(
            " \x1b[1;32m Enabled:             {}",
            details.tbf_header.enabled()
        );

        println!(
            " \x1b[1;32m Sticky:              {}",
            details.tbf_header.sticky()
        );

        println!(
            " \x1b[1;32m Total_Size:          {}\n\n",
            details.tbf_header.total_size()
        );
    }
}

pub async fn print_info(app_details: &mut [AppAttributes], system_details: &mut SystemAttributes) {
    for (i, details) in app_details.iter().enumerate() {
        println!("\n\x1b[0m\x1b[1;35m ┏━━━━━━━━━━━━━━━━┓");
        println!(
            "\x1b[0m\x1b[1;31m ┃ \x1b[0m\x1b[1;32m App_{:<9?} \x1b[0m\x1b[1;31m┃",
            i
        );
        println!("\x1b[0m\x1b[1;33m ┗━━━━━━━━━━━━━━━━┛");
        println!(
            "\n \x1b[1;32m Name:                {}",
            details.tbf_header.get_package_name().unwrap()
        );

        println!(
            " \x1b[1;32m Version:             {}",
            details.tbf_header.get_binary_version()
        );

        println!(
            " \x1b[1;32m Enabled:             {}",
            details.tbf_header.enabled()
        );

        println!(
            " \x1b[1;32m Stricky:             {}",
            details.tbf_header.sticky()
        );

        println!(
            " \x1b[1;32m Total_Size:          {}",
            details.tbf_header.total_size()
        );

        println!(
            " \x1b[1;32m Address in Flash:  {}",
            system_details.appaddr.unwrap()
        );

        println!(
            " \x1b[1;32m    TBF version:    {}",
            details.tbf_header.get_binary_version()
        );

        println!(
            " \x1b[1;32m    header_size:    {}",
            details.tbf_header.header_size()
        );

        println!(
            " \x1b[1;32m    total_size:     {}",
            details.tbf_header.total_size()
        );

        println!(
            " \x1b[1;32m    checksum:       {}",
            details.tbf_header.checksum()
        );

        println!(" \x1b[1;32m    flags:");
        println!(
            " \x1b[1;32m        enabled:        {}",
            details.tbf_header.enabled()
        );

        println!(
            " \x1b[1;32m        sticky:         {}",
            details.tbf_header.sticky()
        );

        println!(" \x1b[1;32m    TVL: Main (1)",);

        println!(
            " \x1b[1;32m        init_fn_offset:             {}",
            details.tbf_header.get_init_function_offset()
        );

        println!(
            " \x1b[1;32m        protected_size:             {}",
            details.tbf_header.get_protected_size()
        );

        println!(
            " \x1b[1;32m        minimum_ram_size:           {}",
            details.tbf_header.get_minimum_app_ram_size()
        );

        println!(" \x1b[1;32m    TVL: Program (9)",);

        println!(
            " \x1b[1;32m        init_fn_offset:             {}",
            details.tbf_header.get_init_function_offset()
        );

        println!(
            " \x1b[1;32m        protected_size:             {}",
            details.tbf_header.get_protected_size()
        );

        println!(
            " \x1b[1;32m        minimum_ram_size:           {}",
            details.tbf_header.get_minimum_app_ram_size()
        );

        println!(
            " \x1b[1;32m        binary_end_offset:          {}",
            details.tbf_header.get_binary_end()
        );

        println!(
            " \x1b[1;32m        app_version:                {}",
            details.tbf_header.get_binary_version()
        );

        println!(" \x1b[1;32m    TVL: Package Name (3)",);

        println!(
            " \x1b[1;32m        package_name:               {}",
            details.tbf_header.get_package_name().unwrap()
        );

        println!(" \x1b[1;32m    TVL: Kernel Version (8)",);

        println!(
            " \x1b[1;32m        kernel_major:               {}",
            details.tbf_header.get_kernel_version().unwrap().0
        );

        println!(
            " \x1b[1;32m        kernel_minor:               {}",
            details.tbf_header.get_kernel_version().unwrap().1,
        );

        println!("\n \x1b[1;32m    Footer");

        let mut total_footer_size: u32 = 0;

        //  Usage of +4 is a result of the structure of the Tock Binary Format (https://book.tockos.org/doc/tock_binary_format)
        //  Because we need the real size of the footer including the type and length.
        for footer_details in details.tbf_footers.iter() {
            total_footer_size += footer_details.size + 4;
        }

        println!(
            " \x1b[1;32m            footer_size:            {}",
            total_footer_size
        );

        for (i, footer_details) in details.tbf_footers.iter().enumerate() {
            println!(" \x1b[1;32m    Footer [{i}] TVL: Credentials");

            println!(
                " \x1b[1;32m        Type:                       {}",
                footer_details.credentials.get_type()
            );

            //  Usage of -4 is a result of the structure of the Tock Binary Format (https://book.tockos.org/doc/tock_binary_format)
            //  Because we only need the size of the credentials without the type and length bytes.
            println!(
                " \x1b[1;32m        Length:                     {}",
                footer_details.size - 4
            );
        }
    }

    println!("\n\n\x1b[1;32m Kernel Attributes");
    println!(
        "\x1b[1;32m     Sentinel:               {:<10}",
        system_details.sentinel.clone().unwrap()
    );
    println!(
        "\x1b[1;32m     Version:                {:<10}",
        system_details.kernel_version.unwrap()
    );
    println!("\x1b[1;32m KATLV: APP Memory");
    println!(
        "\x1b[1;32m     app_memory_start:       {:<10}",
        system_details.app_mem_start.unwrap()
    );
    println!(
        "\x1b[1;32m     app_memory_len:         {:<10}",
        system_details.app_mem_len.unwrap()
    );
    println!("\x1b[1;32m KATLV: Kernel Binary");
    println!(
        "\x1b[1;32m     kernel_binary_start:    {:<10}",
        system_details.kernel_bin_start.unwrap()
    );
    println!(
        "\x1b[1;32m     kernel_binary_len:      {:<10}\n\n",
        system_details.kernel_bin_len.unwrap()
    );
}
