set dotenv-load

version := "0.6.0"

notarize:
    xcrun notarytool submit --apple-id $A_ID --password $PASSWORD --team-id 4JJPCY2A78 hoverpane-{{version}}.dmg --wait

info:
    xcrun notarytool info f7714a27-e17b-4009-b4c6-d38b1b6f3974 --apple-id $A_ID --password $PASSWORD --team-id 4JJPCY2A78

staple:
    xcrun stapler staple ./hoverpane-{{version}}.dmg

sign:
    codesign --force --deep --options runtime --sign "$CERTIFICATE_TYPE: $DEVELOPER_ID" target/release/hoverpane.app

macos: sign
    # Create DMG
    hdiutil create -volname "HoverPane" -srcfolder target/release/hoverpane.app -ov -format UDZO hoverpane-{{version}}.dmg
    # Notarize
    xcrun notarytool submit --apple-id $A_ID --password $PASSWORD --team-id 4JJPCY2A78 ./hoverpane-{{version}}.dmg --wait
    # Staple the notarization
    xcrun stapler staple ./hoverpane-{{version}}.dmg

notarize-info:
    xcrun notarytool info f7714a27-e17b-4009-b4c6-d38b1b6f3974 --apple-id $A_ID --team-id 4JJPCY2A78 --password $PASSWORD
    spctl -a -t exec -v /Users/jarde/Documents/code/web-extension-scraper/target/release/hoverpane.app

landingpage:
    cd landingpage && npm run build 
    wrangler pages deploy landingpage/dist --project-name widget-maker-landing

logs:
    cat ~/Library/Application\ Support/com.jarde.hoverpane/hoverpane.log


release:
    cd hoverpane-app && cargo build --release

final: release
    cd hoverpane-app && cargo packager --release --formats app

version:
    cd hoverpane-app && cargo version


all: release macos version

