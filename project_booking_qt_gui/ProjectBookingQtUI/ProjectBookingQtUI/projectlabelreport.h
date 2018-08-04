#ifndef PROJECTREPORT_H
#define PROJECTREPORT_H
#include "jsonparser.h"

class ProjectLabelReport
{
public:
    ProjectLabelReport();
    static Result< QList<LabelReport> > fromQJsonDocument (QJsonDocument json);
};

#endif // PROJECTREPORT_H
