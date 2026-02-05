import QtQuick
import QtQuick.Controls as QQC2
import QtQuick.Layouts
import org.kde.plasma.plasmoid
import org.kde.kirigami as Kirigami
import org.kde.plasma.plasma5support as Plasma5Support

PlasmoidItem {
    id: root

    preferredRepresentation: compactRepresentation

    compactRepresentation: MouseArea {
        id: compactMouse
        Layout.minimumWidth: Kirigami.Units.iconSizes.small
        Layout.minimumHeight: Kirigami.Units.iconSizes.small
        hoverEnabled: true
        acceptedButtons: Qt.LeftButton | Qt.RightButton
        
        onClicked: function(mouse) {
            if (mouse.button === Qt.RightButton) {
                contextMenu.popup()
            } else {
                executable.exec("razer-settings")
            }
        }

        Kirigami.Icon {
            anchors.fill: parent
            source: "preferences-system-power-management"
        }

        QQC2.ToolTip {
            text: "Razer Control\nClick to open settings\nRight-click for menu"
            visible: compactMouse.containsMouse
            delay: 500
        }

        QQC2.Menu {
            id: contextMenu

            QQC2.MenuItem {
                text: "Open Settings"
                icon.name: "preferences-system"
                onTriggered: executable.exec("razer-settings")
            }

            QQC2.MenuSeparator { }

            QQC2.MenuItem {
                text: "Fan Control"
                icon.name: "fan"
                onTriggered: executable.exec("razer-settings")
            }

            QQC2.MenuItem {
                text: "Power Profiles"
                icon.name: "battery"
                onTriggered: executable.exec("razer-settings")
            }

            QQC2.MenuItem {
                text: "RGB Control"
                icon.name: "colors"
                onTriggered: executable.exec("razer-settings")
            }

            QQC2.MenuItem {
                text: "Battery Health"
                icon.name: "battery-100-charging"
                onTriggered: executable.exec("razer-settings")
            }

            QQC2.MenuSeparator { }

            QQC2.MenuItem {
                text: "Support Development ❤️"
                icon.name: "help-donate"
                onTriggered: Qt.openUrlExternally("https://www.paypal.com/donate/?hosted_button_id=H4SCC24R8KS4A")
            }
        }
    }

    fullRepresentation: Item {
        Layout.minimumWidth: Kirigami.Units.gridUnit * 20
        Layout.minimumHeight: Kirigami.Units.gridUnit * 15

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: Kirigami.Units.largeSpacing

            Kirigami.Heading {
                text: "Razer Control"
                level: 1
            }

            QQC2.Label {
                text: "Click to open Razer Settings"
            }

            Item { Layout.fillHeight: true }

            QQC2.Button {
                text: "Open Settings"
                icon.name: "preferences-system"
                Layout.fillWidth: true
                onClicked: executable.exec("razer-settings")
            }
        }
    }

    Plasma5Support.DataSource {
        id: executable
        engine: "executable"
        connectedSources: []
        
        function exec(cmd) {
            connectSource(cmd)
        }
        
        onNewData: function(sourceName, data) {
            disconnectSource(sourceName)
        }
    }
}
