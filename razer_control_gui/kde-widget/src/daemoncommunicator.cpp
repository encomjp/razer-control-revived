#include "daemoncommunicator.h"
#include <QLocalSocket>
#include <QStandardPaths>
#include <QJsonDocument>
#include <QJsonObject>

DaemonCommunicator::DaemonCommunicator()
    : m_socket(nullptr), m_connected(false)
{
}

DaemonCommunicator::~DaemonCommunicator()
{
    if (m_socket) {
        m_socket->disconnectFromServer();
        delete m_socket;
    }
}

bool DaemonCommunicator::connectToDaemon()
{
    if (m_connected && m_socket && m_socket->state() == QLocalSocket::ConnectedState) {
        return true;
    }

    if (!m_socket) {
        m_socket = new QLocalSocket();
    }

    QString socketPath = QStandardPaths::writableLocation(QStandardPaths::RuntimeLocation)
                       + "/razer-daemon.sock";

    m_socket->connectToServer(socketPath);
    
    if (m_socket->waitForConnected(1000)) {
        m_connected = true;
        return true;
    }

    m_connected = false;
    return false;
}

QString DaemonCommunicator::readResponse()
{
    if (!m_socket || !m_connected) {
        return QString();
    }

    if (m_socket->waitForReadyRead(1000)) {
        return QString::fromUtf8(m_socket->readAll());
    }

    return QString();
}

QString DaemonCommunicator::getDeviceName()
{
    if (!connectToDaemon()) {
        return "Unknown Device";
    }

    m_socket->write("{\"command\": \"GetDeviceName\"}\n");
    m_socket->flush();

    QString response = readResponse();
    
    // Parse JSON response
    QJsonDocument doc = QJsonDocument::fromJson(response.toUtf8());
    if (!doc.isNull() && doc.isObject()) {
        QJsonObject obj = doc.object();
        if (obj.contains("name")) {
            return obj["name"].toString();
        }
    }

    return "Unknown Device";
}

int DaemonCommunicator::getBatteryPercentage()
{
    if (!connectToDaemon()) {
        return 0;
    }

    m_socket->write("{\"command\": \"GetBattery\"}\n");
    m_socket->flush();

    QString response = readResponse();
    
    // Parse JSON response
    QJsonDocument doc = QJsonDocument::fromJson(response.toUtf8());
    if (!doc.isNull() && doc.isObject()) {
        QJsonObject obj = doc.object();
        if (obj.contains("percentage")) {
            return obj["percentage"].toInt();
        }
    }

    return 0;
}

bool DaemonCommunicator::sendCommand(const QString &command)
{
    if (!connectToDaemon()) {
        return false;
    }

    m_socket->write(command.toUtf8());
    m_socket->write("\n");
    m_socket->flush();

    return true;
}
