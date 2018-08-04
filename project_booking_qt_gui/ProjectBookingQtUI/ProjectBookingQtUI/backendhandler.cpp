#include "backendhandler.h"

BackendHandler::BackendHandler()
{

}

Result<QString> BackendHandler::getBackendResponse (QString arguments) {
    QProcess process;
    QString backednCall = arguments;
    process.start(backednCall);
    process.waitForFinished(60000);

    QString errorText = process.readAllStandardError();
    if (!errorText.isEmpty()){
        Err<QString> error(errorText);
        return error;
    }

    QString successText = process.readAllStandardOutput();
    Ok<QString> success(successText);
    return success;
}

Result<QString> BackendHandler::backendApplicationPath () {
    QString backendInterface = "backend/project_booking_backend_gui_interface";
    QFileInfo check_file(backendInterface);
    if (check_file.exists() && check_file.isFile()) {
        Ok<QString> success(backendInterface);
        return success;
    }

    Err<QString> error(backendInterface +" was not found.");
    return error;
}

Result< QList<Data> > BackendHandler::reportForMonth(int month) {
    Result<QString> backendInterface = backendApplicationPath();
    if (backendInterface.hasError) {
        Err< QList<Data> > error(backendInterface.err());
        return error;
    }

    QString command = backendInterface.ok() + " reportForMonth " + QString::number(month);

    Result<QString> response = getBackendResponse(command);
    if (response.hasError)
    {
        Err< QList<Data> > error(response.err());
        return error;
    }
    if (response.ok().isEmpty())
    {
        Err< QList<Data> > error("No resposne received from backend!");
        return error;
    }

    QJsonParseError parse_error;
    QJsonDocument qjson = QJsonDocument::fromJson(response.ok().toUtf8(),&parse_error);
    if (QJsonParseError::NoError!=parse_error.error)
    {
        Err< QList<Data> > error(QString(parse_error.errorString() +" occured while parsing " + response.ok()));
        return error;
    }

    return MonthReport::fromQJsonDocument(qjson);
}

Result< QList<TaskReport> > BackendHandler::standardReport() {

    Result<QString> backendInterface = backendApplicationPath();
    if (backendInterface.hasError) {
        Err< QList<TaskReport> > error(backendInterface.err());
        return error;
    }

    QString command = backendInterface.ok() + " report";

    Result<QString> response = getBackendResponse(command);
    if (response.hasError)
    {
        Err< QList<TaskReport> > error(response.err());
        return error;
    }
    if (response.ok().isEmpty())
    {
        Err< QList<TaskReport> > error("No resposne received from backend!");
        return error;
    }

    QJsonParseError parse_error;
    QJsonDocument qjson = QJsonDocument::fromJson(response.ok().toUtf8(),&parse_error);
    if (QJsonParseError::NoError!=parse_error.error)
    {
        Err< QList<TaskReport> > error(QString(parse_error.errorString() +" occured while parsing " + response.ok()));
        return error;
    }

    return StandardReport::fromQJsonDocument(qjson);
}

Result< QList<LabelReport> > BackendHandler::projectLabelReport() {

    Result<QString> backendInterface = backendApplicationPath();
    if (backendInterface.hasError) {
        Err< QList<LabelReport> > error(backendInterface.err());
        return error;
    }

    QString command = backendInterface.ok() + " reportByLabel";

    Result<QString> response = getBackendResponse(command);
    if (response.hasError)
    {
        Err< QList<LabelReport> > error(response.err());
        return error;
    }
    if (response.ok().isEmpty())
    {
        Err< QList<LabelReport> > error("No resposne received from backend!");
        return error;
    }

    QJsonParseError parse_error;
    QJsonDocument qjson = QJsonDocument::fromJson(response.ok().toUtf8(),&parse_error);
    if (QJsonParseError::NoError!=parse_error.error)
    {
        Err< QList<LabelReport> > error(QString(parse_error.errorString() +" occured while parsing " + response.ok()));
        return error;
    }

    return ProjectLabelReport::fromQJsonDocument(qjson);
}

Result< QString > BackendHandler::clockOut(QString taskName) {
    Result<QString> backendInterface = backendApplicationPath();
    if (backendInterface.hasError)
        return backendInterface;

    QString command = backendInterface.ok() + " clockOut " + taskName;

    return getBackendResponse(command);
}

Result< QString > BackendHandler::clockIn(QString taskName) {
    Result<QString> backendInterface = backendApplicationPath();
    if (backendInterface.hasError)
        return backendInterface;

    QString command = backendInterface.ok() + " clockIn " + taskName;

    return getBackendResponse(command);
}

Result< QString > BackendHandler::addTime(QString taskName, QDateTime timeDate) {
    Result<QString> backendInterface = backendApplicationPath();
    if (backendInterface.hasError)
        return backendInterface;
    QDateTime localDateTime = timeDate.toLocalTime();
    QString dateTimeText = localDateTime.time().toString("hh:mm") + " " + localDateTime.date().toString("dd.MM.yyyy");
    QString command = backendInterface.ok() + " addTime " + taskName + " " + dateTimeText;

    return getBackendResponse(command);
}

Result< QString > BackendHandler::removeTime(QString taskName, QDateTime timeDate) {
    Result<QString> backendInterface = backendApplicationPath();
    if (backendInterface.hasError)
        return backendInterface;
    QDateTime localDateTime = timeDate.toLocalTime();
    QString dateTimeText = localDateTime.time().toString("hh:mm") + " " + localDateTime.date().toString("dd.MM.yyyy");
    QString command = backendInterface.ok() + " removeTime " + taskName + " " + dateTimeText;

    return getBackendResponse(command);
}

Result< QString > BackendHandler::newTask(QString taskName, QString projects) {
    Result<QString> backendInterface = backendApplicationPath();
    if (backendInterface.hasError)
        return backendInterface;
    QString command = backendInterface.ok() + " new " + taskName + " " + projects;

    return getBackendResponse(command);
}

Result< QString > BackendHandler::remove(QString taskName) {
    Result<QString> backendInterface = backendApplicationPath();
    if (backendInterface.hasError)
        return backendInterface;

    QString command = backendInterface.ok() + " delete " + taskName;

    return getBackendResponse(command);
}

Result< QString > BackendHandler::help() {
    Result<QString> backendInterface = backendApplicationPath();
    if (backendInterface.hasError)
        return backendInterface;

    QString command = backendInterface.ok() + " help";

    return getBackendResponse(command);
}

Result< QString > BackendHandler::license() {
    Result<QString> backendInterface = backendApplicationPath();
    if (backendInterface.hasError)
        return backendInterface;

    QString command = backendInterface.ok() + " license";

    return getBackendResponse(command);
}
