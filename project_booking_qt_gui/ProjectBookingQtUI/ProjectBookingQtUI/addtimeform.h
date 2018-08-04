#ifndef ADDTIMEFORM_H
#define ADDTIMEFORM_H
#include <QPushButton>
#include <QDateTimeEdit>
#include <QSignalMapper>
#include "reports.h"

class AddRemoveTimeForm : public QWidget
{
    Q_OBJECT

public:
    explicit AddRemoveTimeForm(Reports *, StatusDisplay *,QWidget *);

public slots:
    void submitAddRequest(QString task);
    void submitRemoveRequest(QString task);
    void trigger(QString task);

private:
    QWidget *parentWidget;
    Reports *reports;
    StatusDisplay *statusDisplay;
    QDateTimeEdit *inputData;
    QPushButton *cancel;
    QPushButton *submitAdd;
    QSignalMapper *submitAddMapper;
    QPushButton *submitRemove;
    QSignalMapper *submitRemoveMapper;
    void close();
};

#endif // ADDTIMEFORM_H
