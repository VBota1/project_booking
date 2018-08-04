#include "standardreport.h"

StandardReport::StandardReport()
{

}

Result< QList<TaskReport> > StandardReport::fromQJsonDocument (QJsonDocument json)
{
    QList<TaskReport> response;

    if ( !json.isArray() )
    {
        Err< QList<TaskReport> > error("Data From backend is not a collection of elements.");
        return error;
    }

    QJsonArray backendDataArray = json.array();

    if (backendDataArray.isEmpty())
    {
        Err< QList<TaskReport> > error("There is no record in the backed response.");
        return error;
    }

    uint arrayIndex = 0;
    while (!backendDataArray.isEmpty())
    {
        TaskReport task = TaskReport();

        QJsonObject arrayElement = backendDataArray.first().toObject();

        Result< QString > fieldResponse = JSonParser::getStringFieldFromJsonArrayElement(arrayElement,"id",arrayIndex);
        if (fieldResponse.hasError) {
            Err< QList<TaskReport> > error(fieldResponse.err());
            return error;
        }
        task.id = fieldResponse.ok();

        fieldResponse = JSonParser::getStringFieldFromJsonArrayElement(arrayElement,"name",arrayIndex);
        if (fieldResponse.hasError) {
            Err< QList<TaskReport> > error(fieldResponse.err());
            return error;
        }
        task.name = fieldResponse.ok();

        fieldResponse = JSonParser::getStringFieldFromJsonArrayElement(arrayElement,"time_spent",arrayIndex);
        if (fieldResponse.hasError) {
            Err< QList<TaskReport> > error(fieldResponse.err());
            return error;
        }
        task.time_spent = fieldResponse.ok();

        Result< QList<QString> > labelList = JSonParser::getLabelsFromJsonElement (arrayElement,arrayIndex);
        if (labelList.hasError) {
            Err< QList<TaskReport> > error(labelList.err());
            return error;
        }
        task.labels = labelList.ok();

        fieldResponse = JSonParser::getStringFieldFromJsonArrayElement(arrayElement,"clock_in_timestamp",arrayIndex);
        if (fieldResponse.hasError) {
            Err< QList<TaskReport> > error(fieldResponse.err());
            return error;
        }
        task.clock_in_timestamp = fieldResponse.ok();

        response.append(task);

        backendDataArray.removeFirst();
        arrayIndex++;
    }
    Ok< QList<TaskReport> > success(response);
    return success;
}

