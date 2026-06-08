while getopts 'dr' OPTION; do
    case "$OPTION" in 
        d)
            echo "Building dev seiche"
            cargo nice-plug bundle seiche
            ;;
        r)
            echo "Building prod seiche"
            cargo nice-plug bundle seiche --release
            ;;
        ?)
            echo "No build version provided. Building prod seiche"
            cargo nice-plug bundle seiche
            ;;
    esac
done
shift "$(($OPTIND -1))"

echo "Removing old release version from VST3 folder."
rm -rf ~/Library/Audio/Plug-Ins/VST3/seiche.vst3

echo "Moving new vst3 version into VST3 folder."
mv target/bundled/seiche.vst3 ~/Library/Audio/Plug-Ins/VST3/seiche.vst3

echo "Removing old release version from CLAP folder."
rm -rf ~/Library/Audio/Plug-Ins/CLAP/seiche.clap

echo "Moving new vst3 version into CLAP folder."
mv target/bundled/seiche.clap ~/Library/Audio/Plug-Ins/CLAP/seiche.clap