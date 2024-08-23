// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use tockloader_lib::attributes::{
    app_attributes::AppAttributes, system_attributes::SystemAttributes,
};

pub async fn print_list(app_details: &mut [AppAttributes]) {
    for (i, temp_data) in app_details.iter().enumerate() {
        println!("\n\x1b[0m\x1b[1;35m ┏━━━━━━━━━━━━━━━━┓");
        println!(
            "\x1b[0m\x1b[1;31m ┃ \x1b[0m\x1b[1;32m App_{:<9?} \x1b[0m\x1b[1;31m┃",
            i
        );
        println!("\x1b[0m\x1b[1;33m ┗━━━━━━━━━━━━━━━━┛");
        println!(
            "\n \x1b[1;32m Name:                {}",
            temp_data.tbf_header.get_package_name().unwrap()
        );

        println!(
            " \x1b[1;32m Version:             {}",
            temp_data.tbf_header.get_binary_version()
        );

        println!(
            " \x1b[1;32m Enabled:             {}",
            temp_data.tbf_header.enabled()
        );

        println!(
            " \x1b[1;32m Sticky:              {}",
            temp_data.tbf_header.sticky()
        );

        println!(
            " \x1b[1;32m Total_Size:          {}\n\n",
            temp_data.tbf_header.total_size()
        );
    }
}

pub async fn print_info(app_details: &mut [AppAttributes], system_details: &mut SystemAttributes) {
    for (i, temp_data) in app_details.iter().enumerate() {
        println!("\n\x1b[0m\x1b[1;35m ┏━━━━━━━━━━━━━━━━┓");
        println!(
            "\x1b[0m\x1b[1;31m ┃ \x1b[0m\x1b[1;32m App_{:<9?} \x1b[0m\x1b[1;31m┃",
            i
        );
        println!("\x1b[0m\x1b[1;33m ┗━━━━━━━━━━━━━━━━┛");
        println!(
            "\n \x1b[1;32m Name:                {}",
            temp_data.tbf_header.get_package_name().unwrap()
        );

        println!(
            " \x1b[1;32m Version:             {}",
            temp_data.tbf_header.get_binary_version()
        );

        println!(
            " \x1b[1;32m Enabled:             {}",
            temp_data.tbf_header.enabled()
        );

        println!(
            " \x1b[1;32m Stricky:             {}",
            temp_data.tbf_header.sticky()
        );

        println!(
            " \x1b[1;32m Total_Size:          {}",
            temp_data.tbf_header.total_size()
        );

        println!(
            " \x1b[1;32m Address in Flash:  {:<10}",
            system_details.appaddr.unwrap()
        );

        println!(
            " \x1b[1;32m    TBF version:    {:<10}",
            temp_data.tbf_header.get_binary_version()
        );

        println!(
            " \x1b[1;32m    header_size:    {:<10}",
            temp_data.tbf_header.header_size()
        );

        println!(
            " \x1b[1;32m    total_size:     {:<10}",
            temp_data.tbf_header.total_size()
        );

        println!(
            " \x1b[1;32m    checksum:       {:<10}",
            temp_data.tbf_header.checksum()
        );

        println!(" \x1b[1;32m    flags:");
        println!(
            " \x1b[1;32m        enabled:        {:<10}",
            temp_data.tbf_header.enabled()
        );

        println!(
            " \x1b[1;32m        sticky:         {:<10}",
            temp_data.tbf_header.sticky()
        );

        println!(" \x1b[1;32m    TVL: Main (1)",);

        println!(
            " \x1b[1;32m        init_fn_offset:             {:<10}",
            temp_data.tbf_header.get_init_function_offset()
        );

        println!(
            " \x1b[1;32m        protected_size:             {:<10}",
            temp_data.tbf_header.get_protected_size()
        );

        println!(
            " \x1b[1;32m        minimum_ram_size:           {:<10}",
            temp_data.tbf_header.get_minimum_app_ram_size()
        );

        println!(" \x1b[1;32m    TVL: Program (9)",);

        println!(
            " \x1b[1;32m        init_fn_offset:             {:<10}",
            temp_data.tbf_header.get_init_function_offset()
        );

        println!(
            " \x1b[1;32m        protected_size:             {:<10}",
            temp_data.tbf_header.get_protected_size()
        );

        println!(
            " \x1b[1;32m        minimum_ram_size:           {:<10}",
            temp_data.tbf_header.get_minimum_app_ram_size()
        );

        println!(
            " \x1b[1;32m        binary_end_offset:          {:<10}",
            temp_data.tbf_header.get_binary_end()
        );

        println!(
            " \x1b[1;32m        app_version:                {:<10}",
            temp_data.tbf_header.get_binary_version()
        );

        println!(" \x1b[1;32m    TVL: Package Name (3)",);

        println!(
            " \x1b[1;32m        package_name:               {:<10}",
            temp_data.tbf_header.get_package_name().unwrap()
        );

        println!(" \x1b[1;32m    TVL: Kernel Version (8)",);

        println!(
            " \x1b[1;32m        kernel_major:               {:<10}",
            temp_data.tbf_header.get_kernel_version().unwrap().0,
        );

        println!(
            " \x1b[1;32m        kernel_minor:               {:<10}",
            temp_data.tbf_header.get_kernel_version().unwrap().1,
        );

        println!(
            " \x1b[1;32m        kernel version:             {}.{}",
            temp_data.tbf_header.get_kernel_version().unwrap().0,
            temp_data.tbf_header.get_kernel_version().unwrap().1
        );

        //TODO(NegrilaRares): Remake the whole footer part
        //take multiple into consideration

        // println!("\n \x1b[1;32m    Footer");
        //
        // println!(
        //     " \x1b[1;32m        footer_size:            {:<10}",
        //     temp_data.footer_size.unwrap()
        // );
        //
        // println!(" \x1b[1;32m    Footer TVL: Credentials");
        //
        // println!(
        //     " \x1b[1;32m        Length:                 {:<10}\n\n",
        //     temp_data.footer_size.unwrap() - 8
        // );
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
