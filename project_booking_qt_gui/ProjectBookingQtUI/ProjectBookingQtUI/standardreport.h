#ifndef STANDARDREPORT_H
#define STANDARDREPORT_H
#include "jsonparser.h"

class StandardReport
{
public:
    StandardReport();
    static Result< QList<TaskReport> > fromQJsonDocument (QJsonDocument);
};

#endif // STANDARDREPORT_H
