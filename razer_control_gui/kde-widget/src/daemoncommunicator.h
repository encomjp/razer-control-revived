#ifndef DAEMONCOMMUNICATOR_H
#define DAEMONCOMMUNICATOR_H

#include <QString>
#include <QLocalSocket>

class DaemonCommunicator
{
public:
    DaemonCommunicator();
    ~DaemonCommunicator();

    QString getDeviceName();
    int getBatteryPercentage();
    bool sendCommand(const QString &command);
    
private:
    bool connectToDaemon();
    QString readResponse();
    
    QLocalSocket *m_socket;
    bool m_connected;
};

#endif // DAEMONCOMMUNICATOR_H
