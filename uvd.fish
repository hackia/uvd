set -l commands install uninstall reinstall search update upgrade verify info publish login logout list build export
complete -c uvd -f

complete -c uvd -n "not __fish_seen_subcommand_from $commands" -a "install" -d "install an universal verified disc" -r
complete -c uvd -n "not __fish_seen_subcommand_from $commands" -a "uninstall" -d "uninstall an universal verified disc" -r
complete -c uvd -n "not __fish_seen_subcommand_from $commands" -a "reinstall" -d "reinstall an universal verified disc" -r
complete -c uvd -n "not __fish_seen_subcommand_from $commands" -a "search" -d "search an universal verified disc" -r
complete -c uvd -n "not __fish_seen_subcommand_from $commands" -a "update" -d "update an universal verified disc" -r
complete -c uvd -n "not __fish_seen_subcommand_from $commands" -a "upgrade" -d "upgrade all universal verified disc"
complete -c uvd -n "not __fish_seen_subcommand_from $commands" -a "verify" -d "verify if a universal disc is valid" -r
complete -c uvd -n "not __fish_seen_subcommand_from $commands" -a "info" -d "Get info for an universal disc" -r
complete -c uvd -n "not __fish_seen_subcommand_from $commands" -a "publish" -d "publish an universal disc"
complete -c uvd -n "not __fish_seen_subcommand_from $commands" -a "login" -d "login to the universal verified disc hub"
complete -c uvd -n "not __fish_seen_subcommand_from $commands" -a "list" -d "list all installed universal verified discs"
complete -c uvd -n "not __fish_seen_subcommand_from $commands" -a "build" -d "build a universal verified disc from source"
complete -c uvd -n "not __fish_seen_subcommand_from $commands" -a "logout" -d "logout to the universal verified disc hub"
complete -c uvd -n "not __fish_seen_subcommand_from $commands" -a "export" -d "export the universal verified disc to usb drive" -f