#ifndef JSONPARSER_H
#define JSONPARSER_H
#include <QJsonDocument>
#include <QJsonArray>
#include <QJsonObject>
#include "err.h"
#include "ok.h"

class JSonParser
{
public:
    JSonParser();
    static Result< QString > getStringFieldFromJsonArrayElement (QJsonObject arrayElement, QString field, uint arrayIndex);
    static Result< QList<QString> > getLabelsFromJsonElement (QJsonObject arrayElement, uint arrayIndex);
};

#endif // JSONPARSER_H
