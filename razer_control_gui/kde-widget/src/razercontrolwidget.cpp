#include "razercontrolwidget.h"
#include <KConfigDialog>
#include <KConfigGroup>
#include <KSharedConfig>
#include <KAuthorized>
#include <KLocalizedString>
#include <QStandardPaths>
#include <QDesktopServices>
#include <QUrl>
#include <QProcess>
#include <QFile>
#include <QFileInfo>
#include <KPluginFactory>

K_PLUGIN_CLASS_WITH_JSON(RazerControlWidget, "metadata.json")

RazerControlWidget::RazerControlWidget(QObject *parent)
    : Plasma::Applet(parent)
    , m_batteryPercentage(0)
    , m_isMinimized(false)
    , m_startMinimized(true)
    , m_autoStartEnabled(true)
{
    setHasConfigurationInterface(true);
    setConfigurationRequired(false);
}

RazerControlWidget::~RazerControlWidget()
{
}

void RazerControlWidget::init()
{
    Plasma::Applet::init();
    
    readConfiguration();
    setupAutoStart();
    setupConnections();
    
    // Set up update timer for battery info
    connect(&m_updateTimer, &QTimer::timeout, this, &RazerControlWidget::updateBatteryInfo);
    
    // Read configuration for refresh interval (default 2 seconds)
    KConfigGroup cg = config();
    int refreshInterval = cg.readEntry("RefreshInterval", 2) * 1000;
    m_updateTimer.setInterval(refreshInterval);
    m_updateTimer.start();
    
    updateDeviceInfo();
    updateBatteryInfo();
}

void RazerControlWidget::setupConnections()
{
    setAction("razer-settings", i18n("Open Razer Settings"), QStringLiteral("preferences-system"));
    setAction("minimize-app", i18n("Minimize Application"), QStringLiteral("window-minimize"));
    setAction("toggle-autostart", i18n("Toggle Auto-Start"), QStringLiteral("system-reboot"));
    
    connect(action("razer-settings"), &QAction::triggered, this, &RazerControlWidget::openSettings);
    connect(action("minimize-app"), &QAction::triggered, this, &RazerControlWidget::minimizeApp);
}

void RazerControlWidget::readConfiguration()
{
    KConfigGroup cg = config();
    m_startMinimized = cg.readEntry("StartMinimized", true);
    m_autoStartEnabled = cg.readEntry("EnableAutoStart", true);
}

void RazerControlWidget::setupAutoStart()
{
    if (!m_autoStartEnabled) {
        return;
    }
    
    QString autoStartPath = QStandardPaths::writableLocation(QStandardPaths::GenericConfigLocation)
                          + "/autostart/razer-settings.desktop";
    
    if (!QFile::exists(autoStartPath)) {
        QFile desktopFile(autoStartPath);
        if (desktopFile.open(QIODevice::WriteOnly | QIODevice::Text)) {
            QTextStream out(&desktopFile);
            out << "[Desktop Entry]\n";
            out << "Type=Application\n";
            out << "Name=Razer Control\n";
            out << "Exec=razer-settings" << (m_startMinimized ? " --minimized" : "") << "\n";
            out << "Icon=preferences-system-power-management\n";
            out << "Categories=Utility;System;\n";
            out << "Terminal=false\n";
            desktopFile.close();
        }
    } else if (m_startMinimized) {
        // Update existing entry to include --minimized flag
        QFile desktopFile(autoStartPath);
        if (desktopFile.open(QIODevice::ReadOnly | QIODevice::Text)) {
            QString content = desktopFile.readAll();
            desktopFile.close();
            
            if (!content.contains("--minimized")) {
                content.replace("Exec=razer-settings", "Exec=razer-settings --minimized");
                if (desktopFile.open(QIODevice::WriteOnly | QIODevice::Text)) {
                    QTextStream out(&desktopFile);
                    out << content;
                    desktopFile.close();
                }
            }
        }
    }
}

void RazerControlWidget::updateDeviceInfo()
{
    // This would connect to the daemon and fetch device info
    // For now, using placeholder
    QString newDeviceName = m_daemonComm.getDeviceName();
    if (newDeviceName != m_deviceName) {
        m_deviceName = newDeviceName;
        emit deviceNameChanged();
    }
}

void RazerControlWidget::updateBatteryInfo()
{
    // This would connect to the daemon and fetch battery info
    // For now, using placeholder
    int newBattery = m_daemonComm.getBatteryPercentage();
    if (newBattery != m_batteryPercentage) {
        m_batteryPercentage = newBattery;
        emit batteryPercentageChanged();
    }
}

void RazerControlWidget::openSettings()
{
    QProcess::startDetached("razer-settings");
}

void RazerControlWidget::minimizeApp()
{
    m_isMinimized = !m_isMinimized;
    emit minimizedStateChanged();
    
    QProcess::startDetached("pkill", QStringList() << "-f" << "razer-settings");
}

void RazerControlWidget::setAutoStart(bool enabled)
{
    m_autoStartEnabled = enabled;
    KConfigGroup cg = config();
    cg.writeEntry("EnableAutoStart", enabled);
    cg.sync();
    
    setupAutoStart();
}

void RazerControlWidget::setStartMinimized(bool enabled)
{
    m_startMinimized = enabled;
    KConfigGroup cg = config();
    cg.writeEntry("StartMinimized", enabled);
    cg.sync();
    
    setupAutoStart();
}

void RazerControlWidget::configChanged()
{
    Plasma::Applet::configChanged();
    readConfiguration();
}

void RazerControlWidget::createConfigurationInterface(KConfigDialog *parent)
{
    QWidget *generalPage = new QWidget(parent);
    
    QVBoxLayout *layout = new QVBoxLayout(generalPage);
    
    QCheckBox *startMinimizedCheckBox = new QCheckBox(i18n("Start application minimized on boot"), generalPage);
    startMinimizedCheckBox->setChecked(m_startMinimized);
    layout->addWidget(startMinimizedCheckBox);
    
    QCheckBox *autoStartCheckBox = new QCheckBox(i18n("Auto-start on system boot"), generalPage);
    autoStartCheckBox->setChecked(m_autoStartEnabled);
    layout->addWidget(autoStartCheckBox);
    
    QCheckBox *showBatteryCheckBox = new QCheckBox(i18n("Show battery percentage in widget"), generalPage);
    showBatteryCheckBox->setChecked(config().readEntry("ShowBatteryPercentage", true));
    layout->addWidget(showBatteryCheckBox);
    
    layout->addStretch();
    
    parent->addPage(generalPage, i18n("General"), icon());
    
    connect(startMinimizedCheckBox, &QCheckBox::stateChanged, parent, &KConfigDialog::settingsChanged);
    connect(autoStartCheckBox, &QCheckBox::stateChanged, parent, &KConfigDialog::settingsChanged);
    connect(showBatteryCheckBox, &QCheckBox::stateChanged, parent, &KConfigDialog::settingsChanged);
}

#include "razercontrolwidget.moc"
