#ifndef NEWTASKFORM_H
#define NEWTASKFORM_H
#include <QPushButton>
#include <QDateTimeEdit>
#include <QSignalMapper>
#include <QLineEdit>
#include "reports.h"

class NewTaskForm : public QWidget
{
    Q_OBJECT

public:
    explicit NewTaskForm(Reports *, StatusDisplay *,QWidget *);

public slots:
    void trigger(QString task);

private:
    QWidget *parentWidget;
    Reports *reports;
    StatusDisplay *statusDisplay;
    QLineEdit *taskName;
    QLineEdit *labels;
    QPushButton *cancel;
    QPushButton *submit;
    void submitRequest();
    void close();
};

#endif // NEWTASKFORM_H
