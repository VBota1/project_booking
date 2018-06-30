#ifndef BACKENDHANDLER_H
#define BACKENDHANDLER_H
#include <QString>
#include <QList>

struct taskDayData {
    QString task;
    QString time_spent;
    QList<QString> labels;
};

struct monthReport {
    QString date;
    QList<taskDayData> tasks;
};

class backendHandler
{
public:
    backendHandler();
    QString getBackendResponse (QString arguments);
};

#endif // BACKENDHANDLER_H
