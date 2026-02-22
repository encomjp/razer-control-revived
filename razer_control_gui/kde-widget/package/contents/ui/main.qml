import QtQuick
import QtQuick.Controls as QQC2
import QtQuick.Layouts
import org.kde.plasma.plasmoid
import org.kde.kirigami as Kirigami
import org.kde.plasma.plasma5support as Plasma5Support

PlasmoidItem {
    id: root

    // --- Sensor values ---
    property string cpuTemp: "--"
    property string cpuName: "CPU"
    property string dgpuTemp: "--"
    property string dgpuName: "dGPU"
    property string igpuTemp: "--"
    property string igpuName: "iGPU"
    property string fanSpeed: "--"
    property string batteryPct: "--"
    property string acPower: "--"
    property string batteryStatus: "--"
    property string dgpuPower: "--"
    property string dgpuUtil: "--"
    property string igpuPower: "--"
    property string igpuUtil: "--"
    property string cpuPower: "--"
    property string cpuUtil: "--"
    property string batteryWatts: "--"
    property string cpuFreq: "--"
    property string igpuFreq: "--"
    property string dgpuFreq: "--"

    // RAPL tracking
    property real _lastRaplUj: 0
    property real _lastRaplTime: 0

    // CPU stat tracking (for utilization delta)
    property real _lastCpuIdle: 0
    property real _lastCpuTotal: 0

    // Write guard: skip poll updates for settings briefly after a write
    property real _lastWriteTime: 0

    // --- Daemon settings ---
    property string powerProfile: "--"
    property string brightness: "--"
    property string logoMode: "--"
    property string bhoStatus: "--"

    // ac state helper for writes
    property string acState: acPower === "1" ? "ac" : "bat"

    // Fan RPM presets for cycling
    property var fanPresets: [0, 3000, 3500, 4000, 4500, 5000]

    // --- Update checker ---
    readonly property string currentVersion: "0.2.8"
    property string latestVersion: ""
    property string latestUrl: ""
    property bool updateDismissed: false

    switchWidth: Kirigami.Units.gridUnit * 12
    switchHeight: Kirigami.Units.gridUnit * 8

    // --- Compact representation (panel) ---
    compactRepresentation: MouseArea {
        id: compactMouse
        Layout.minimumWidth: compactRow.implicitWidth + Kirigami.Units.largeSpacing * 2
        Layout.minimumHeight: Kirigami.Units.iconSizes.small * 1.5
        hoverEnabled: true
        onClicked: root.expanded = !root.expanded

        RowLayout {
            id: compactRow
            anchors.centerIn: parent
            spacing: Kirigami.Units.largeSpacing
            Kirigami.Icon {
                source: "com.github.encomjp.razercontrol"
                Layout.preferredWidth: Kirigami.Units.iconSizes.smallMedium
                Layout.preferredHeight: Kirigami.Units.iconSizes.smallMedium
            }
            QQC2.Label {
                text: cpuTemp !== "--" ? cpuTemp + "\u00B0" : ""
                font.pixelSize: Kirigami.Theme.smallFont.pixelSize
                visible: cpuTemp !== "--"
            }
        }

        QQC2.ToolTip {
            text: {
                var l = ["Razer Control"];
                if (cpuTemp !== "--") l.push("CPU: " + cpuTemp + "\u00B0C");
                if (dgpuTemp !== "--") l.push("dGPU: " + dgpuTemp + "\u00B0C");
                if (fanSpeed !== "--") l.push("Fan: " + (fanSpeed === "0" ? "Auto" : fanSpeed + " RPM"));
                if (batteryPct !== "--") l.push("Battery: " + batteryPct + "%");
                return l.join("\n");
            }
            visible: compactMouse.containsMouse
            delay: 300
        }
    }

    // --- Full representation (desktop / expanded) ---
    fullRepresentation: Item {
        Layout.minimumWidth: Kirigami.Units.gridUnit * 22
        Layout.maximumWidth: Kirigami.Units.gridUnit * 28
        Layout.preferredWidth: Kirigami.Units.gridUnit * 25
        Layout.preferredHeight: mainCol.implicitHeight + Kirigami.Units.largeSpacing * 2
        implicitHeight: mainCol.implicitHeight + Kirigami.Units.largeSpacing * 2

        ColumnLayout {
            id: mainCol
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.top: parent.top
            anchors.margins: Kirigami.Units.smallSpacing
            spacing: Kirigami.Units.largeSpacing

            // ===== HEADER (clickable to open app) =====
            MouseArea {
                Layout.fillWidth: true
                implicitHeight: headerRow.implicitHeight
                cursorShape: Qt.PointingHandCursor
                onClicked: root.launchApp()

                RowLayout {
                    id: headerRow
                    anchors.fill: parent
                    spacing: Kirigami.Units.largeSpacing

                    Kirigami.Icon {
                        source: "com.github.encomjp.razercontrol"
                        Layout.preferredWidth: Kirigami.Units.iconSizes.large
                        Layout.preferredHeight: Kirigami.Units.iconSizes.large
                    }
                    Kirigami.Heading { text: "Razer Control"; level: 3; font.weight: Font.DemiBold }
                    Item { Layout.fillWidth: true }
                    Kirigami.Icon {
                        source: "go-next-symbolic"
                        Layout.preferredWidth: 16; Layout.preferredHeight: 16
                        opacity: 0.4
                    }
                }
            }

            Kirigami.Separator { Layout.fillWidth: true; Layout.topMargin: Kirigami.Units.largeSpacing }

            // ===== SYSTEM MONITOR (merged temps + power + util) =====
            Rectangle {
                Layout.fillWidth: true
                implicitHeight: monitorCol.implicitHeight + Kirigami.Units.largeSpacing * 2
                radius: Kirigami.Units.largeSpacing
                color: Qt.rgba(Kirigami.Theme.backgroundColor.r, Kirigami.Theme.backgroundColor.g, Kirigami.Theme.backgroundColor.b, 0.3)
                border.width: 1
                border.color: Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.1)

                ColumnLayout {
                    id: monitorCol
                    anchors.fill: parent
                    anchors.margins: Kirigami.Units.smallSpacing
                    spacing: Kirigami.Units.largeSpacing

                    // CPU
                    RowLayout {
                        Layout.fillWidth: true
                        spacing: Kirigami.Units.largeSpacing
                        Kirigami.Icon { source: "cpu-symbolic"; Layout.preferredWidth: 18; Layout.preferredHeight: 18; opacity: 0.9 }
                        QQC2.Label { 
                            text: cpuName; 
                            Layout.fillWidth: true; 
                            elide: Text.ElideRight; 
                            font.pixelSize: Kirigami.Theme.smallFont.pixelSize; 
                            font.weight: Font.DemiBold
                        }
                        QQC2.Label {
                            text: cpuTemp !== "--" ? cpuTemp + "°C" : ""
                            font.bold: true; 
                            font.pixelSize: Kirigami.Theme.defaultFont.pixelSize
                            Layout.preferredWidth: 48; 
                            horizontalAlignment: Text.AlignRight
                            color: cpuTemp !== "--" ? (parseFloat(cpuTemp) > 90 ? Kirigami.Theme.negativeTextColor : parseFloat(cpuTemp) > 75 ? Kirigami.Theme.neutralTextColor : Kirigami.Theme.positiveTextColor) : Kirigami.Theme.textColor
                        }
                        QQC2.Label { 
                            text: cpuFreq !== "--" ? cpuFreq + " GHz" : ""; 
                            Layout.preferredWidth: 62; 
                            horizontalAlignment: Text.AlignRight; 
                            opacity: 0.7;
                            font.pixelSize: Kirigami.Theme.smallFont.pixelSize
                        }
                        Rectangle {
                            Layout.preferredWidth: 1
                            Layout.preferredHeight: 14
                            color: Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.2)
                            visible: cpuPower !== "--"
                        }
                        QQC2.Label { 
                            text: cpuPower !== "--" ? cpuPower + " W" : ""; 
                            Layout.preferredWidth: 48; 
                            horizontalAlignment: Text.AlignRight; 
                            font.weight: Font.DemiBold;
                            font.pixelSize: Kirigami.Theme.smallFont.pixelSize
                        }
                        QQC2.Label { 
                            text: cpuUtil !== "--" ? cpuUtil + "%" : ""; 
                            Layout.preferredWidth: 32; 
                            horizontalAlignment: Text.AlignRight; 
                            opacity: 0.7;
                            font.pixelSize: Kirigami.Theme.smallFont.pixelSize
                        }
                    }

                    // iGPU
                    RowLayout {
                        Layout.fillWidth: true
                        spacing: Kirigami.Units.largeSpacing
                        visible: igpuTemp !== "--" || igpuPower !== "--"
                        Kirigami.Icon { source: "video-display-symbolic"; Layout.preferredWidth: 18; Layout.preferredHeight: 18; opacity: 0.9 }
                        QQC2.Label { 
                            text: igpuName; 
                            Layout.fillWidth: true; 
                            elide: Text.ElideRight; 
                            font.pixelSize: Kirigami.Theme.smallFont.pixelSize;
                            font.weight: Font.DemiBold
                        }
                        QQC2.Label {
                            text: igpuTemp !== "--" ? igpuTemp + "°C" : ""
                            font.bold: true; 
                            font.pixelSize: Kirigami.Theme.defaultFont.pixelSize
                            Layout.preferredWidth: 48; 
                            horizontalAlignment: Text.AlignRight
                            color: igpuTemp !== "--" ? (parseFloat(igpuTemp) > 90 ? Kirigami.Theme.negativeTextColor : parseFloat(igpuTemp) > 75 ? Kirigami.Theme.neutralTextColor : Kirigami.Theme.positiveTextColor) : Kirigami.Theme.textColor
                        }
                        QQC2.Label { 
                            text: igpuFreq !== "--" ? igpuFreq + " MHz" : ""; 
                            Layout.preferredWidth: 62; 
                            horizontalAlignment: Text.AlignRight; 
                            opacity: 0.7;
                            font.pixelSize: Kirigami.Theme.smallFont.pixelSize
                        }
                        Rectangle {
                            Layout.preferredWidth: 1
                            Layout.preferredHeight: 14
                            color: Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.2)
                            visible: igpuPower !== "--"
                        }
                        QQC2.Label { 
                            text: igpuPower !== "--" ? igpuPower + " W" : ""; 
                            Layout.preferredWidth: 48; 
                            horizontalAlignment: Text.AlignRight; 
                            font.weight: Font.DemiBold;
                            font.pixelSize: Kirigami.Theme.smallFont.pixelSize
                        }
                        QQC2.Label { 
                            text: igpuUtil !== "--" ? igpuUtil + "%" : ""; 
                            Layout.preferredWidth: 32; 
                            horizontalAlignment: Text.AlignRight; 
                            opacity: 0.7;
                            font.pixelSize: Kirigami.Theme.smallFont.pixelSize
                        }
                    }

                    // dGPU
                    RowLayout {
                        Layout.fillWidth: true
                        spacing: Kirigami.Units.largeSpacing
                        Kirigami.Icon { 
                            source: "video-display-symbolic"; 
                            Layout.preferredWidth: 18; 
                            Layout.preferredHeight: 18; 
                            opacity: dgpuTemp !== "--" ? 0.9 : 0.4
                        }
                        QQC2.Label { 
                            text: dgpuName; 
                            Layout.fillWidth: true; 
                            elide: Text.ElideRight; 
                            font.pixelSize: Kirigami.Theme.smallFont.pixelSize;
                            font.weight: Font.DemiBold
                            opacity: dgpuTemp !== "--" ? 1.0 : 0.5
                        }
                        QQC2.Label {
                            text: dgpuTemp !== "--" ? dgpuTemp + "°C" : "Off"
                            font.bold: dgpuTemp !== "--"
                            font.pixelSize: Kirigami.Theme.defaultFont.pixelSize
                            Layout.preferredWidth: 48; 
                            horizontalAlignment: Text.AlignRight
                            color: dgpuTemp !== "--" ? (parseFloat(dgpuTemp) > 85 ? Kirigami.Theme.negativeTextColor : parseFloat(dgpuTemp) > 70 ? Kirigami.Theme.neutralTextColor : Kirigami.Theme.positiveTextColor) : Kirigami.Theme.disabledTextColor
                        }
                        QQC2.Label { 
                            text: dgpuFreq !== "--" ? dgpuFreq + " MHz" : ""; 
                            Layout.preferredWidth: 62; 
                            horizontalAlignment: Text.AlignRight; 
                            opacity: 0.7;
                            font.pixelSize: Kirigami.Theme.smallFont.pixelSize
                        }
                        Rectangle {
                            Layout.preferredWidth: 1
                            Layout.preferredHeight: 14
                            color: Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.2)
                            visible: dgpuPower !== "--"
                        }
                        QQC2.Label { 
                            text: dgpuPower !== "--" ? dgpuPower + " W" : ""; 
                            Layout.preferredWidth: 48; 
                            horizontalAlignment: Text.AlignRight; 
                            font.weight: Font.DemiBold;
                            font.pixelSize: Kirigami.Theme.smallFont.pixelSize
                        }
                        QQC2.Label { 
                            text: dgpuUtil !== "--" ? dgpuUtil + "%" : ""; 
                            Layout.preferredWidth: 32; 
                            horizontalAlignment: Text.AlignRight; 
                            opacity: 0.7;
                            font.pixelSize: Kirigami.Theme.smallFont.pixelSize
                        }
                    }
                }
            }

            // ===== BATTERY BAR =====
            Rectangle {
                Layout.fillWidth: true
                implicitHeight: batteryCol.implicitHeight + Kirigami.Units.largeSpacing * 2
                radius: Kirigami.Units.largeSpacing
                visible: batteryPct !== "--"
                color: Qt.rgba(Kirigami.Theme.backgroundColor.r, Kirigami.Theme.backgroundColor.g, Kirigami.Theme.backgroundColor.b, 0.3)
                border.width: 1
                border.color: Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.1)

                ColumnLayout {
                    id: batteryCol
                    anchors.fill: parent
                    anchors.margins: Kirigami.Units.smallSpacing
                    spacing: Kirigami.Units.largeSpacing

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: Kirigami.Units.largeSpacing
                        Kirigami.Icon {
                            source: batteryStatus === "Charging" ? "battery-charging" : batteryStatus === "Not charging" ? "battery-level-80-charging" : "battery"
                            Layout.preferredWidth: 18; Layout.preferredHeight: 18
                        }
                        QQC2.Label {
                            text: {
                                if (batteryStatus === "Charging") return "Battery – Charging";
                                if (batteryStatus === "Not charging") return "Battery – Full (Limit)";
                                if (batteryStatus === "Full") return "Battery – Full";
                                return "Battery – Discharging";
                            }
                            font.weight: Font.DemiBold
                        }
                        Item { Layout.fillWidth: true }
                        QQC2.Label {
                            visible: batteryWatts !== "--" && batteryWatts !== "0.0" && (batteryStatus === "Charging" || batteryStatus === "Discharging")
                            text: batteryStatus === "Charging" ? "+" + batteryWatts + " W" : "−" + batteryWatts + " W"
                            font.bold: true
                            font.pixelSize: Kirigami.Theme.defaultFont.pixelSize
                            color: batteryStatus === "Charging" ? Kirigami.Theme.positiveTextColor : Kirigami.Theme.neutralTextColor
                        }
                        QQC2.Label { 
                            text: batteryPct + "%"; 
                            font.bold: true;
                            font.pixelSize: Kirigami.Theme.defaultFont.pixelSize
                        }
                    }
                    // Custom battery bar in Razer green
                    Item {
                        Layout.fillWidth: true
                        implicitHeight: 6
                        Rectangle {
                            anchors.fill: parent
                            radius: 3
                            color: Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.1)
                        }
                        Rectangle {
                            width: parent.width * (batteryPct !== "--" ? Math.min(parseInt(batteryPct), 100) / 100 : 0)
                            height: parent.height
                            radius: 3
                            color: batteryStatus === "Charging" ? "#44d62c" :
                                   parseInt(batteryPct) <= 15 ? Kirigami.Theme.negativeTextColor :
                                   parseInt(batteryPct) <= 30 ? Kirigami.Theme.neutralTextColor : "#44d62c"
                            Behavior on width { NumberAnimation { duration: 500; easing.type: Easing.OutCubic } }
                        }
                    }
                }
            }

            Kirigami.Separator { Layout.fillWidth: true; Layout.topMargin: Kirigami.Units.largeSpacing }

            // ===== SETTINGS (single grouped card) =====
            Rectangle {
                Layout.fillWidth: true
                implicitHeight: settingsCol.implicitHeight + Kirigami.Units.largeSpacing * 2
                radius: Kirigami.Units.largeSpacing
                color: Qt.rgba(Kirigami.Theme.backgroundColor.r, Kirigami.Theme.backgroundColor.g, Kirigami.Theme.backgroundColor.b, 0.3)
                border.width: 1
                border.color: Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.1)

                ColumnLayout {
                    id: settingsCol
                    anchors.fill: parent
                    anchors.margins: Kirigami.Units.smallSpacing
                    spacing: 0

                    // Profile
                    MouseArea {
                        id: profileMouse
                        Layout.fillWidth: true
                        implicitHeight: profileRow.implicitHeight + Kirigami.Units.largeSpacing
                        hoverEnabled: true; cursorShape: Qt.PointingHandCursor
                        onClicked: {
                            root._lastWriteTime = Date.now();
                            var cur = parseInt(root.powerProfile);
                            var next = isNaN(cur) ? 0 : (cur + 1) % 4;
                            executable.exec("razer-cli write power " + root.acState + " " + next);
                            root.powerProfile = next.toString();
                            refreshTimer.restart();
                        }
                        Rectangle {
                            anchors.fill: parent; radius: 4
                            color: profileMouse.containsMouse ? Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.2) : "transparent"
                        }
                        RowLayout {
                            id: profileRow; anchors.fill: parent; anchors.leftMargin: Kirigami.Units.largeSpacing; anchors.rightMargin: Kirigami.Units.largeSpacing
                            Kirigami.Icon { source: "system-run"; Layout.preferredWidth: 18; Layout.preferredHeight: 18 }
                            QQC2.Label { text: "Profile"; font.weight: Font.DemiBold; font.pixelSize: Kirigami.Theme.smallFont.pixelSize }
                            Item { Layout.fillWidth: true }
                            QQC2.Label {
                                text: { switch(powerProfile) { case "0": return "Balanced"; case "1": return "Gaming"; case "2": return "Creator"; case "3": return "Silent"; case "4": return "Custom"; default: return "--"; } }
                                font.bold: true; font.pixelSize: Kirigami.Theme.smallFont.pixelSize; color: Kirigami.Theme.positiveTextColor
                            }
                            Kirigami.Icon { source: "go-next-symbolic"; Layout.preferredWidth: 12; Layout.preferredHeight: 12; opacity: 0.4 }
                        }
                    }

                    Rectangle { Layout.fillWidth: true; Layout.leftMargin: Kirigami.Units.largeSpacing; Layout.rightMargin: Kirigami.Units.largeSpacing; implicitHeight: 1; color: Kirigami.Theme.alternateBackgroundColor }

                    // Fan
                    MouseArea {
                        id: fanMouse
                        Layout.fillWidth: true
                        implicitHeight: fanRow.implicitHeight + Kirigami.Units.largeSpacing
                        hoverEnabled: true; cursorShape: Qt.PointingHandCursor
                        onClicked: {
                            root._lastWriteTime = Date.now();
                            var cur = parseInt(root.fanSpeed);
                            var idx = 0;
                            if (!isNaN(cur)) {
                                for (var i = 0; i < root.fanPresets.length; i++) {
                                    if (root.fanPresets[i] === cur) { idx = i; break; }
                                }
                            }
                            var next = (idx + 1) % root.fanPresets.length;
                            executable.exec("razer-cli write fan " + root.acState + " " + root.fanPresets[next]);
                            root.fanSpeed = root.fanPresets[next].toString();
                            refreshTimer.restart();
                        }
                        Rectangle {
                            anchors.fill: parent; radius: 4
                            color: fanMouse.containsMouse ? Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.2) : "transparent"
                        }
                        RowLayout {
                            id: fanRow; anchors.fill: parent; anchors.leftMargin: Kirigami.Units.largeSpacing; anchors.rightMargin: Kirigami.Units.largeSpacing
                            Kirigami.Icon { source: "speedometer-symbolic"; Layout.preferredWidth: 18; Layout.preferredHeight: 18 }
                            QQC2.Label { text: "Fan"; font.weight: Font.DemiBold; font.pixelSize: Kirigami.Theme.smallFont.pixelSize }
                            Item { Layout.fillWidth: true }
                            QQC2.Label {
                                text: fanSpeed === "--" ? "--" : (fanSpeed === "0" ? "Auto" : fanSpeed + " RPM")
                                font.bold: true; font.pixelSize: Kirigami.Theme.smallFont.pixelSize
                            }
                            Kirigami.Icon { source: "go-next-symbolic"; Layout.preferredWidth: 12; Layout.preferredHeight: 12; opacity: 0.4 }
                        }
                    }

                    Rectangle { Layout.fillWidth: true; Layout.leftMargin: Kirigami.Units.largeSpacing; Layout.rightMargin: Kirigami.Units.largeSpacing; implicitHeight: 1; color: Kirigami.Theme.alternateBackgroundColor }

                    // KB Brightness
                    MouseArea {
                        id: brightMouse
                        Layout.fillWidth: true
                        implicitHeight: brightRow.implicitHeight + Kirigami.Units.largeSpacing
                        hoverEnabled: true; cursorShape: Qt.PointingHandCursor
                        onClicked: {
                            root._lastWriteTime = Date.now();
                            var steps = [0, 25, 50, 75, 100];
                            var cur = parseInt(root.brightness);
                            var idx = 0;
                            for (var i = 0; i < steps.length; i++) { if (steps[i] === cur) { idx = i; break; } }
                            var next = steps[(idx + 1) % steps.length];
                            executable.exec("razer-cli write brightness " + root.acState + " " + next);
                            root.brightness = next.toString();
                            refreshTimer.restart();
                        }
                        Rectangle {
                            anchors.fill: parent; radius: 4
                            color: brightMouse.containsMouse ? Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.2) : "transparent"
                        }
                        RowLayout {
                            id: brightRow; anchors.fill: parent; anchors.leftMargin: Kirigami.Units.largeSpacing; anchors.rightMargin: Kirigami.Units.largeSpacing
                            Kirigami.Icon { source: "brightness-high-symbolic"; Layout.preferredWidth: 18; Layout.preferredHeight: 18 }
                            QQC2.Label { text: "KB Brightness"; font.weight: Font.DemiBold; font.pixelSize: Kirigami.Theme.smallFont.pixelSize }
                            Item { Layout.fillWidth: true }
                            QQC2.Label {
                                text: brightness === "0" ? "Off" : brightness !== "--" ? brightness + "%" : "--"
                                font.bold: true; font.pixelSize: Kirigami.Theme.smallFont.pixelSize
                                color: brightness === "0" ? Kirigami.Theme.disabledTextColor : Kirigami.Theme.textColor
                            }
                            Kirigami.Icon { source: "go-next-symbolic"; Layout.preferredWidth: 12; Layout.preferredHeight: 12; opacity: 0.4 }
                        }
                    }

                    Rectangle { Layout.fillWidth: true; Layout.leftMargin: Kirigami.Units.largeSpacing; Layout.rightMargin: Kirigami.Units.largeSpacing; implicitHeight: 1; color: Kirigami.Theme.alternateBackgroundColor }

                    // Logo
                    MouseArea {
                        id: logoMouse
                        Layout.fillWidth: true
                        implicitHeight: logoRow.implicitHeight + Kirigami.Units.largeSpacing
                        hoverEnabled: true; cursorShape: Qt.PointingHandCursor
                        onClicked: {
                            root._lastWriteTime = Date.now();
                            var cur = parseInt(root.logoMode);
                            var next = isNaN(cur) ? 0 : (cur + 1) % 3;
                            executable.exec("razer-cli write logo " + root.acState + " " + next);
                            root.logoMode = next.toString();
                            refreshTimer.restart();
                        }
                        Rectangle {
                            anchors.fill: parent; radius: 4
                            color: logoMouse.containsMouse ? Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.2) : "transparent"
                        }
                        RowLayout {
                            id: logoRow; anchors.fill: parent; anchors.leftMargin: Kirigami.Units.largeSpacing; anchors.rightMargin: Kirigami.Units.largeSpacing
                            Kirigami.Icon { source: "preferences-desktop-display-color"; Layout.preferredWidth: 18; Layout.preferredHeight: 18 }
                            QQC2.Label { text: "Logo"; font.weight: Font.DemiBold; font.pixelSize: Kirigami.Theme.smallFont.pixelSize }
                            Item { Layout.fillWidth: true }
                            QQC2.Label {
                                text: { switch(logoMode) { case "0": return "Off"; case "1": return "On"; case "2": return "Breathing"; default: return "--"; } }
                                font.bold: true; font.pixelSize: Kirigami.Theme.smallFont.pixelSize
                                color: logoMode === "0" ? Kirigami.Theme.disabledTextColor : Kirigami.Theme.positiveTextColor
                            }
                            Kirigami.Icon { source: "go-next-symbolic"; Layout.preferredWidth: 12; Layout.preferredHeight: 12; opacity: 0.4 }
                        }
                    }

                    Rectangle { Layout.fillWidth: true; Layout.leftMargin: Kirigami.Units.largeSpacing; Layout.rightMargin: Kirigami.Units.largeSpacing; implicitHeight: 1; color: Kirigami.Theme.alternateBackgroundColor }

                    // Charge Limit
                    MouseArea {
                        id: bhoMouse
                        Layout.fillWidth: true
                        implicitHeight: bhoRow.implicitHeight + Kirigami.Units.largeSpacing
                        hoverEnabled: true; cursorShape: Qt.PointingHandCursor
                        onClicked: {
                            root._lastWriteTime = Date.now();
                            var isOn = root.bhoStatus.indexOf("On") >= 0;
                            if (isOn) { executable.exec("razer-cli write bho off"); root.bhoStatus = "Off"; }
                            else { executable.exec("razer-cli write bho on 80"); root.bhoStatus = "On/80%"; }
                            refreshTimer.restart();
                        }
                        Rectangle {
                            anchors.fill: parent; radius: 4
                            color: bhoMouse.containsMouse ? Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.2) : "transparent"
                        }
                        RowLayout {
                            id: bhoRow; anchors.fill: parent; anchors.leftMargin: Kirigami.Units.largeSpacing; anchors.rightMargin: Kirigami.Units.largeSpacing
                            Kirigami.Icon { source: "battery-good-charging-symbolic"; Layout.preferredWidth: 18; Layout.preferredHeight: 18 }
                            QQC2.Label { text: "Charge Limit"; font.weight: Font.DemiBold; font.pixelSize: Kirigami.Theme.smallFont.pixelSize }
                            Item { Layout.fillWidth: true }
                            QQC2.Label {
                                visible: bhoStatus.indexOf("On") >= 0
                                text: {
                                    var m = bhoStatus.match(/(\d+)/);
                                    return m ? m[1] + "%" : "";
                                }
                                opacity: 0.6; font.pixelSize: Kirigami.Theme.smallFont.pixelSize
                            }
                            QQC2.Label {
                                text: bhoStatus.indexOf("On") >= 0 ? "On" : "Off"
                                font.bold: true; font.pixelSize: Kirigami.Theme.smallFont.pixelSize
                                color: bhoStatus.indexOf("On") >= 0 ? Kirigami.Theme.positiveTextColor : Kirigami.Theme.disabledTextColor
                            }
                            Kirigami.Icon { source: "go-next-symbolic"; Layout.preferredWidth: 12; Layout.preferredHeight: 12; opacity: 0.4 }
                        }
                    }
                }
            }

            // Delayed re-read after a write action
            Timer {
                id: refreshTimer
                interval: 1000
                onTriggered: sensorSource.connectSource(sensorSource.sensorCmd)
            }

            // ===== UPDATE AVAILABLE BANNER =====
            Rectangle {
                Layout.fillWidth: true
                implicitHeight: updateRow.implicitHeight + Kirigami.Units.smallSpacing * 2
                radius: Kirigami.Units.largeSpacing
                visible: root.latestVersion !== "" && root.latestVersion !== root.currentVersion && !root.updateDismissed
                color: Qt.rgba(Kirigami.Theme.neutralTextColor.r, Kirigami.Theme.neutralTextColor.g, Kirigami.Theme.neutralTextColor.b, 0.1)
                border.width: 1
                border.color: Qt.rgba(Kirigami.Theme.neutralTextColor.r, Kirigami.Theme.neutralTextColor.g, Kirigami.Theme.neutralTextColor.b, 0.25)

                RowLayout {
                    id: updateRow
                    anchors.fill: parent
                    anchors.margins: Kirigami.Units.smallSpacing
                    spacing: Kirigami.Units.smallSpacing

                    Kirigami.Icon {
                        source: "update-none"
                        Layout.preferredWidth: 16; Layout.preferredHeight: 16
                    }
                    QQC2.Label {
                        text: "Update available: " + root.latestVersion
                        font.pixelSize: Kirigami.Theme.smallFont.pixelSize
                        Layout.fillWidth: true
                    }
                    MouseArea {
                        Layout.preferredWidth: openLabel.implicitWidth + Kirigami.Units.smallSpacing * 2
                        Layout.preferredHeight: openLabel.implicitHeight + 4
                        cursorShape: Qt.PointingHandCursor
                        onClicked: Qt.openUrlExternally(root.latestUrl)
                        QQC2.Label {
                            id: openLabel
                            anchors.centerIn: parent
                            text: "View"
                            font.pixelSize: Kirigami.Theme.smallFont.pixelSize
                            font.weight: Font.DemiBold
                            color: Kirigami.Theme.linkColor
                        }
                    }
                    MouseArea {
                        Layout.preferredWidth: 16; Layout.preferredHeight: 16
                        cursorShape: Qt.PointingHandCursor
                        onClicked: root.updateDismissed = true
                        Kirigami.Icon {
                            anchors.fill: parent
                            source: "dialog-close"
                            opacity: 0.5
                        }
                    }
                }
            }
    }
    }

    // ===== APP LAUNCHER =====
    function launchApp() {
        executable.exec("gdbus call --session --dest com.encomjp.razer-settings --object-path /com/encomjp/razer_settings --method org.gtk.Application.Activate '[]' 2>/dev/null || razer-settings")
    }

    // ===== COMMAND EXECUTOR =====
    Plasma5Support.DataSource {
        id: executable
        engine: "executable"
        connectedSources: []
        function exec(cmd) { connectSource(cmd) }
        onNewData: function(sourceName, data) { disconnectSource(sourceName) }
    }

    // ===== UPDATE CHECKER (every 6 hours) =====
    function isNewerVersion(remote, local) {
        var r = remote.replace(/^v/, "").split(".").map(function(x) { return parseInt(x) || 0; });
        var l = local.replace(/^v/, "").split(".").map(function(x) { return parseInt(x) || 0; });
        for (var i = 0; i < Math.max(r.length, l.length); i++) {
            var rv = i < r.length ? r[i] : 0;
            var lv = i < l.length ? l[i] : 0;
            if (rv > lv) return true;
            if (rv < lv) return false;
        }
        return false;
    }

    Plasma5Support.DataSource {
        id: updateChecker
        engine: "executable"
        interval: 21600000  // 6 hours in ms
        connectedSources: [updateCmd]
        property string updateCmd: "curl -sf --max-time 10 https://api.github.com/repos/encomjp/razer-control-revived/releases/latest"

        onNewData: function(sourceName, data) {
            var stdout = data["stdout"];
            if (!stdout) return;
            try {
                var json = JSON.parse(stdout);
                var tag = json.tag_name || "";
                var url = json.html_url || "https://github.com/encomjp/razer-control-revived/releases";
                if (tag && root.isNewerVersion(tag, root.currentVersion)) {
                    root.latestVersion = tag;
                    root.latestUrl = url;
                    root.updateDismissed = false;
                } else {
                    root.latestVersion = "";
                }
            } catch(e) {
                // Silently ignore parse errors
            }
        }
    }

    // ===== SENSOR + SETTINGS POLLER =====
    Plasma5Support.DataSource {
        id: sensorSource
        engine: "executable"
        interval: 2000
        connectedSources: [sensorCmd]

        property string sensorCmd: "bash -c '" +
            "head -1 /proc/stat | awk \"{t=0; for(i=2;i<=NF;i++) t+=\\$i; print \\\"CPU_STAT=\\\"\\$5\\\":\\\"t}\"; " +
            "for f in /sys/class/hwmon/hwmon*/name; do " +
            "  n=$(cat $f 2>/dev/null); " +
            "  case $n in coretemp|k10temp|zenpower) " +
            "    echo CPU_TEMP=$(cat $(dirname $f)/temp1_input 2>/dev/null); break;; " +
            "  esac; " +
            "done; " +
            "for f in /sys/class/hwmon/hwmon*/name; do " +
            "  n=$(cat $f 2>/dev/null); " +
            "  case $n in amdgpu) " +
            "    d=$(dirname $f); " +
            "    echo IGPU_TEMP=$(cat $d/temp1_input 2>/dev/null); " +
            "    echo IGPU_POWER=$(cat $d/power1_average 2>/dev/null); " +
            "    echo IGPU_UTIL=$(cat $d/device/gpu_busy_percent 2>/dev/null); " +
            "    break;; " +
            "  esac; " +
            "done; " +
            "echo DGPU_TEMP=$(nvidia-smi --query-gpu=temperature.gpu --format=csv,noheader,nounits 2>/dev/null); " +
            "echo DGPU_POWER=$(nvidia-smi --query-gpu=power.draw --format=csv,noheader,nounits 2>/dev/null); " +
            "echo DGPU_UTIL=$(nvidia-smi --query-gpu=utilization.gpu --format=csv,noheader,nounits 2>/dev/null); " +
            "echo BATTERY=$(cat /sys/class/power_supply/BAT0/capacity 2>/dev/null || cat /sys/class/power_supply/BAT1/capacity 2>/dev/null); " +
            "echo AC=$(cat /sys/class/power_supply/AC0/online 2>/dev/null || cat /sys/class/power_supply/ADP0/online 2>/dev/null || cat /sys/class/power_supply/ADP1/online 2>/dev/null); " +
            "echo BAT_STATUS=$(cat /sys/class/power_supply/BAT0/status 2>/dev/null || cat /sys/class/power_supply/BAT1/status 2>/dev/null); " +
            "c=$(cat /sys/class/power_supply/BAT0/current_now 2>/dev/null); " +
            "v=$(cat /sys/class/power_supply/BAT0/voltage_now 2>/dev/null); " +
            "[ -n \"$c\" ] && [ -n \"$v\" ] && [ \"$c\" -gt 0 ] && echo BAT_POWER_UW=$((c*v/1000000)); " +
            "for d in /sys/class/powercap/intel-rapl*/intel-rapl:0/energy_uj /sys/class/powercap/intel-rapl:0/energy_uj /sys/devices/virtual/powercap/intel-rapl/intel-rapl:0/energy_uj /sys/class/powercap/amd-rapl*/amd-rapl:0/energy_uj /sys/class/powercap/amd-rapl:0/energy_uj; do " +
            "  [ -r \"$d\" ] && echo RAPL_UJ=$(cat \"$d\" 2>/dev/null) && break; " +
            "done; " +
            "ac_on=$(cat /sys/class/power_supply/AC0/online 2>/dev/null || cat /sys/class/power_supply/ADP0/online 2>/dev/null || cat /sys/class/power_supply/ADP1/online 2>/dev/null); " +
            "_st=bat; [ \"$ac_on\" = \"1\" ] && _st=ac; " +
            "echo FAN_SPEED=$(razer-cli read fan $_st 2>/dev/null | grep -oP \"[0-9]+\" | tail -1); " +
            "echo POWER_PROFILE=$(razer-cli read power $_st 2>/dev/null | grep -oP \"[0-9]+\" | head -1); " +
            "echo BRIGHTNESS=$(razer-cli read brightness $_st 2>/dev/null | grep -oP \"[0-9]+\" | tail -1); " +
            "echo LOGO=$(razer-cli read logo $_st 2>/dev/null | grep -oP \"[0-9]+\" | tail -1); " +
            "bho=$(razer-cli read bho 2>/dev/null); " +
            "if echo $bho | grep -qi on; then " +
            "  thr=$(echo $bho | grep -oP \"[0-9]+\" | tail -1); " +
            "  echo BHO=On/$thr%; " +
            "elif echo $bho | grep -qi off; then " +
            "  echo BHO=Off; " +
            "fi; " +
            "cn=$(grep -m1 \"model name\" /proc/cpuinfo | cut -d: -f2 | sed \"s/^ //; s/ with Radeon Graphics//; s/ w\\/.*//; s/ 16-Core Processor//\"); " +
            "echo CPU_NAME=$cn; " +
            "ig=$(grep -m1 \"model name\" /proc/cpuinfo | sed -nE \"s/.* (Radeon [0-9]+M).*/\\1/p\"); " +
            "[ -z \"$ig\" ] && ig=$(lspci 2>/dev/null | grep -iE \"VGA|Display|3D\" | grep -iv nvidia | head -1 | sed -E \"s/.*: //; s/ \\(rev .*//; s/Advanced Micro Devices, Inc\\. \\[AMD\\/ATI\\] //; s/Intel Corporation //; s/.*\\[Radeon ([0-9]+M) \\/ [0-9]+M\\].*/Radeon \\1/; s/.*\\[Radeon ([0-9]+M)\\].*/Radeon \\1/\"); " +
            "dg=$(nvidia-smi --query-gpu=name --format=csv,noheader 2>/dev/null | head -1 | sed \"s/ Laptop GPU//\"); " +
            "[ -n \"$ig\" ] && echo IGPU_NAME=$ig; " +
            "[ -n \"$dg\" ] && echo DGPU_NAME=$dg; " +
            "echo CPU_FREQ=$(cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq 2>/dev/null); " +
            "echo IGPU_FREQ=$(cat /sys/class/drm/card*/device/pp_dpm_sclk 2>/dev/null | grep \"\\*\" | sed -E \"s/.*: ([0-9]+)Mhz.*/\\1/\"); " +
            "echo DGPU_FREQ=$(nvidia-smi --query-gpu=clocks.current.graphics --format=csv,noheader,nounits 2>/dev/null); " +
            "'"

        onNewData: function(sourceName, data) {
            var stdout = data["stdout"];
            if (!stdout) return;

            var lines = stdout.split("\n");
            for (var i = 0; i < lines.length; i++) {
                var line = lines[i].trim();
                if (line === "") continue;
                var eq = line.indexOf("=");
                if (eq < 1) continue;
                var key = line.substring(0, eq);
                var val = line.substring(eq + 1).trim();
                if (val === "") continue;

                var writeGuard = (Date.now() - root._lastWriteTime) < 2500;

                switch (key) {
                    case "CPU_TEMP":
                        var t = parseInt(val);
                        if (!isNaN(t)) cpuTemp = Math.round(t / 1000).toString();
                        break;
                    case "DGPU_TEMP":
                        if (!isNaN(parseInt(val))) dgpuTemp = parseInt(val).toString();
                        break;
                    case "IGPU_TEMP":
                        var it = parseInt(val);
                        if (!isNaN(it)) igpuTemp = Math.round(it / 1000).toString();
                        break;
                    case "IGPU_POWER":
                        var ip = parseFloat(val);
                        if (!isNaN(ip)) igpuPower = (ip / 1000000).toFixed(1);
                        break;
                    case "IGPU_UTIL":
                        if (!isNaN(parseInt(val))) igpuUtil = parseInt(val).toString();
                        break;
                    case "FAN_SPEED":
                        if (!writeGuard && !isNaN(parseInt(val))) fanSpeed = parseInt(val).toString();
                        break;
                    case "BATTERY":
                        if (!isNaN(parseInt(val))) batteryPct = parseInt(val).toString();
                        break;
                    case "AC":
                        acPower = val;
                        break;
                    case "DGPU_POWER":
                        if (!isNaN(parseFloat(val))) dgpuPower = parseFloat(val).toFixed(1);
                        break;
                    case "DGPU_UTIL":
                        if (!isNaN(parseInt(val))) dgpuUtil = parseInt(val).toString();
                        break;
                    case "POWER_PROFILE":
                        if (!writeGuard && !isNaN(parseInt(val))) powerProfile = parseInt(val).toString();
                        break;
                    case "BRIGHTNESS":
                        if (!writeGuard && !isNaN(parseInt(val))) brightness = parseInt(val).toString();
                        break;
                    case "LOGO":
                        if (!writeGuard && !isNaN(parseInt(val))) logoMode = parseInt(val).toString();
                        break;
                    case "BHO":
                        if (!writeGuard) bhoStatus = val;
                        break;
                    case "BAT_STATUS":
                        batteryStatus = val;
                        break;
                    case "BAT_POWER_UW":
                        var pw = parseFloat(val);
                        if (!isNaN(pw)) batteryWatts = (pw / 1000000).toFixed(1);
                        break;
                    case "RAPL_UJ":
                        var uj = parseFloat(val);
                        if (!isNaN(uj)) {
                            var now = Date.now() / 1000.0;
                            if (root._lastRaplUj > 0 && root._lastRaplTime > 0) {
                                var dE = uj - root._lastRaplUj;
                                var dT = now - root._lastRaplTime;
                                if (dE < 0) dE += 4294967296000000;
                                if (dT > 0.5 && dT < 10) {
                                    cpuPower = (dE / dT / 1000000).toFixed(1);
                                }
                            }
                            root._lastRaplUj = uj;
                            root._lastRaplTime = now;
                        }
                        break;
                    case "CPU_STAT":
                        var parts = val.split(":");
                        if (parts.length === 2) {
                            var idle = parseFloat(parts[0]);
                            var total = parseFloat(parts[1]);
                            if (!isNaN(idle) && !isNaN(total) && root._lastCpuTotal > 0) {
                                var dIdle = idle - root._lastCpuIdle;
                                var dTotal = total - root._lastCpuTotal;
                                if (dTotal > 0) {
                                    cpuUtil = Math.round(100 * (1 - dIdle / dTotal)).toString();
                                }
                            }
                            root._lastCpuIdle = idle;
                            root._lastCpuTotal = total;
                        }
                        break;
                    case "CPU_NAME":
                        if (val !== "") cpuName = val;
                        break;
                    case "IGPU_NAME":
                        if (val !== "") igpuName = val;
                        break;
                    case "DGPU_NAME":
                        if (val !== "") dgpuName = val;
                        break;
                    case "CPU_FREQ":
                        var cf = parseFloat(val);
                        if (!isNaN(cf)) cpuFreq = (cf / 1000000).toFixed(1);
                        break;
                    case "IGPU_FREQ":
                        if (!isNaN(parseInt(val))) igpuFreq = parseInt(val).toString();
                        break;
                    case "DGPU_FREQ":
                        if (!isNaN(parseInt(val))) dgpuFreq = parseInt(val).toString();
                        break;
                }
            }
        }
    }
}
