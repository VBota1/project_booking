#include "backendhandler.h"
#include <QProcess>

backendHandler::backendHandler()
{

}

QString getBackendResponse (QString arguments) {
    QProcess process;
    QString application_call = "";
    QString backednCall = application_call + arguments;
    process.start(backednCall);
    process.waitForFinished(60000);

    return process.readAllStandardOutput();
    //QString stderr = process.readAllStandardError();
}
