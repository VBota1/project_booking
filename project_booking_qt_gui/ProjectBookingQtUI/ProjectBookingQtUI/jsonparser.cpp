#include "jsonparser.h"

JSonParser::JSonParser()
{

}

Result< QString > JSonParser::getStringFieldFromJsonArrayElement (QJsonObject arrayElement, QString filedName, uint arrayIndex)
{
    if (!arrayElement.contains(filedName))
    {
        Err< QString > error(filedName + "field is missing from the array entry " + QString::number(arrayIndex));
        return error;
    }
    if (!arrayElement[filedName].isString())
    {
        Err< QString > error(filedName + "field from array entry " + QString::number(arrayIndex) + "is not string");
        return error;
    }

    Ok< QString > success (arrayElement[filedName].toString());
    return success;
}

Result< QList<QString> > JSonParser::getLabelsFromJsonElement (QJsonObject arrayElement, uint arrayIndex) {

    QList<QString> response;

    QString labelListFiledName = "labels";
    if (!arrayElement.contains(labelListFiledName))
    {
        Err< QList<QString> > error(labelListFiledName + "field is missing from the array entry " + QString::number(arrayIndex));
        return error;
    }
    if (!arrayElement[labelListFiledName].isArray())
    {
        Err< QList<QString> > error(labelListFiledName + "field from array entry " + QString::number(arrayIndex) + "is not an array");
        return error;
    }

    QJsonArray labelList = arrayElement[labelListFiledName].toArray();

    uint labelIndex = 0;
    while (!labelList.isEmpty())
    {
        if ( !labelList.first().isString() ){
            Err< QList<QString> > error("label from index "+QString::number(labelIndex)+" is not in string format.");
            return error;
        }

        response.push_back(labelList.first().toString());

        labelList.removeFirst();
        labelIndex++;
    }

    Ok< QList<QString> > success(response);
    return success;

}
