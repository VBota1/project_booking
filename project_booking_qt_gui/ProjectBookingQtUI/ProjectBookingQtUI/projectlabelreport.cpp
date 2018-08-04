#include "projectlabelreport.h"

ProjectLabelReport::ProjectLabelReport()
{

}

Result< QList<LabelReport> > ProjectLabelReport::fromQJsonDocument (QJsonDocument json)
{
    QList<LabelReport> response;

    if ( !json.isArray() )
    {
        Err< QList<LabelReport> > error("Data From backend is not a collection of elements.");
        return error;
    }

    QJsonArray backendDataArray = json.array();

    if (backendDataArray.isEmpty())
    {
        Err< QList<LabelReport> > error("There is no record in the backed response.");
        return error;
    }

    uint arrayIndex = 0;
    while (!backendDataArray.isEmpty())
    {
        LabelReport project = LabelReport();

        QJsonObject arrayElement = backendDataArray.first().toObject();

        Result< QString >  fieldResponse = JSonParser::getStringFieldFromJsonArrayElement(arrayElement,"label",arrayIndex);
        if (fieldResponse.hasError) {
            Err< QList<LabelReport> > error(fieldResponse.err());
            return error;
        }
        project.label = fieldResponse.ok();

        fieldResponse = JSonParser::getStringFieldFromJsonArrayElement(arrayElement,"time_spent",arrayIndex);
        if (fieldResponse.hasError) {
            Err< QList<LabelReport> > error(fieldResponse.err());
            return error;
        }
        project.time_spent = fieldResponse.ok();

        response.append(project);

        backendDataArray.removeFirst();
        arrayIndex++;
    }

    Ok< QList<LabelReport> > success(response);
    return success;
}
