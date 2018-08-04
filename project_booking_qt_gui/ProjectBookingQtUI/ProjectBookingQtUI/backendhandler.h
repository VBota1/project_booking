#ifndef BACKENDHANDLER_H
#define BACKENDHANDLER_H
#include <QProcess>
#include <QFileInfo>
#include <QDateTime>
#include <QDate>
#include <QTime>
#include "monthreport.h"
#include "standardreport.h"
#include "projectlabelreport.h"

class BackendHandler
{
public:
    BackendHandler();
    Result< QList<Data> > reportForMonth(int month);
    Result< QList<TaskReport> > standardReport();
    Result< QList<LabelReport> > projectLabelReport();
    Result< QString > clockOut(QString taskName);
    Result< QString > clockIn(QString taskName);
    Result< QString > addTime(QString taskName, QDateTime timeDate);
    Result< QString > removeTime(QString taskName, QDateTime timeDate);
    Result< QString > newTask(QString taskName, QString projects);
    Result< QString > remove(QString taskName);
    Result< QString > help();
    Result< QString > license();
private:
    Result<QString> backendApplicationPath ();
    Result<QString> getBackendResponse (QString arguments);
};

#endif // BACKENDHANDLER_H
