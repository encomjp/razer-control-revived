#ifndef RAZERCONTROLWIDGET_H
#define RAZERCONTROLWIDGET_H

#include <Plasma/Applet>
#include <QTimer>
#include <QLocalSocket>
#include "daemoncommunicator.h"

class RazerControlWidget : public Plasma::Applet
{
    Q_OBJECT
    Q_PROPERTY(QString deviceName READ deviceName NOTIFY deviceNameChanged)
    Q_PROPERTY(int batteryPercentage READ batteryPercentage NOTIFY batteryPercentageChanged)
    Q_PROPERTY(bool isMinimized READ isMinimized NOTIFY minimizedStateChanged)

public:
    explicit RazerControlWidget(QObject *parent = nullptr);
    ~RazerControlWidget();

    QString deviceName() const { return m_deviceName; }
    int batteryPercentage() const { return m_batteryPercentage; }
    bool isMinimized() const { return m_isMinimized; }

public slots:
    void updateDeviceInfo();
    void updateBatteryInfo();
    void openSettings();
    void minimizeApp();
    void setAutoStart(bool enabled);
    void setStartMinimized(bool enabled);

signals:
    void deviceNameChanged();
    void batteryPercentageChanged();
    void minimizedStateChanged();

protected:
    void configChanged() override;
    void createConfigurationInterface(KConfigDialog *parent) override;
    void init() override;

private:
    void setupConnections();
    void readConfiguration();
    void setupAutoStart();

    DaemonCommunicator m_daemonComm;
    QTimer m_updateTimer;
    
    QString m_deviceName;
    int m_batteryPercentage;
    bool m_isMinimized;
    bool m_startMinimized;
    bool m_autoStartEnabled;
};

#endif // RAZERCONTROLWIDGET_H
