#ifndef REPORTFORMATS
#define REPORTFORMATS
#include <QList>
#include <QString>

struct TaskDay {
    QString task;
    QString time_spent;
    QList<QString> labels;
};

struct Data {
    QString date;
    QList<TaskDay> tasks;
};

struct TaskReport {
    QString id;
    QString name;
    QString time_spent;
    QList<QString> labels;
    QString clock_in_timestamp;
};

struct LabelReport {
    QString label;
    QString time_spent;
};

#endif // REPORTFORMATS

