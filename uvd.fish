set -l commands install uninstall reinstall search
complete -c uvd -f

complete -c uvd -n "not __fish_seen_subcommand_from $commands" -a "install" -d "install an universal verified disc" -r
complete -c uvd -n "not __fish_seen_subcommand_from $commands" -a "uninstall" -d "uninstall an universal verified disc" -r
complete -c uvd -n "not __fish_seen_subcommand_from $commands" -a "reinstall" -d "reinstall an universal verified disc" -r
complete -c uvd -n "not __fish_seen_subcommand_from $commands" -a "search" -d "search an universal verified disc" -r
complete -c uvd -n "not __fish_seen_subcommand_from $commands" -a "update" -d "update an universal verified disc" -r
complete -c uvd -n "not __fish_seen_subcommand_from $commands" -a "upgrade" -d "upgrade all universal verified disc"
