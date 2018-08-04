#include "monthreport.h"

MonthReport::MonthReport()
{

}

Result< QList<Data> > MonthReport::fromQJsonDocument (QJsonDocument json) {
    QList<Data> response;

    if ( !json.isArray() )
    {
        Err< QList<Data> > error("Data From backend is not a collection of elements.");
        return error;
    }

    QJsonArray backendDataArray = json.array();

    if (backendDataArray.isEmpty())
    {
        Err< QList<Data> > error("There is no record in the backed response.");
        return error;
    }

    uint arrayIndex = 0;
    while (!backendDataArray.isEmpty())
    {
        Data newData = Data();

        QJsonObject arrayElement = backendDataArray.first().toObject();

        Result< QString > fieldResponse = JSonParser::getStringFieldFromJsonArrayElement(arrayElement,"date",arrayIndex);
        if (fieldResponse.hasError) {
            Err< QList<Data> > error(fieldResponse.err());
            return error;
        }
        newData.date = fieldResponse.ok();

        Result< QList<TaskDay> > taskListResponse = getTasksFromJsonArrayElement(arrayElement,arrayIndex);
        if(taskListResponse.hasError){
            Err< QList<Data> > error(taskListResponse.err());
            return error;
        }
        newData.tasks = taskListResponse.ok();

        response.push_back(newData);

        backendDataArray.removeFirst();
        arrayIndex++;
    }

    Ok< QList<Data> > success(response);
    return success;
}

Result< QList<TaskDay> > MonthReport::getTasksFromJsonArrayElement (QJsonObject arrayElement, uint arrayIndex) {

    QList<TaskDay> response;

    QString taskListFiledName = "tasks";
    if (!arrayElement.contains(taskListFiledName))
    {
        Err< QList<TaskDay> > error(taskListFiledName + "field is missing from the array entry " + QString::number(arrayIndex));
        return error;
    }
    if (!arrayElement[taskListFiledName].isArray())
    {
        Err< QList<TaskDay> > error(taskListFiledName + "field from array entry " + QString::number(arrayIndex) + "is not an array");
        return error;
    }

    QJsonArray taskList = arrayElement[taskListFiledName].toArray();

    uint taskIndex = 0;
    while (!taskList.isEmpty())
    {
        TaskDay newTask = TaskDay();

        QJsonObject taskJsonElement = taskList.first().toObject();

        Result< QString > fieldResponse = JSonParser::getStringFieldFromJsonArrayElement(taskJsonElement,"task",taskIndex);
        if (fieldResponse.hasError) {
            Err< QList<TaskDay> > error(fieldResponse.err());
            return error;
        }
        newTask.task = fieldResponse.ok();

        fieldResponse = JSonParser::getStringFieldFromJsonArrayElement(taskJsonElement,"time_spent",taskIndex);
        if (fieldResponse.hasError) {
            Err< QList<TaskDay> > error(fieldResponse.err());
            return error;
        }
        newTask.time_spent = fieldResponse.ok();

        Result< QList<QString> > labelList = JSonParser::getLabelsFromJsonElement (taskJsonElement,taskIndex);
        if (labelList.hasError) {
            Err< QList<TaskDay> > error(labelList.err());
            return error;
        }
        newTask.labels = labelList.ok();

        response.push_back(newTask);

        taskList.removeFirst();
        taskIndex++;
    }

    Ok< QList<TaskDay> > success(response);
    return success;
}

