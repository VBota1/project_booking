#ifndef MONTHREPORT_H
#define MONTHREPORT_H
#include "jsonparser.h"

class MonthReport
{
private:
    static Result< QList<TaskDay> > getTasksFromJsonArrayElement (QJsonObject arrayElement, uint arrayIndex);
public:
    MonthReport();
    static Result< QList<Data> > fromQJsonDocument (QJsonDocument);
};

#endif // MONTHREPORT_H
