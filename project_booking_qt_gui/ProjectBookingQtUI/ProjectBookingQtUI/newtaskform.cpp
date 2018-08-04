#include "newtaskform.h"


NewTaskForm::NewTaskForm(Reports *referencedReports, StatusDisplay *status,QWidget *wparent) : QWidget(wparent)
{
    parentWidget=wparent;
    reports = referencedReports;
    statusDisplay = status;
}

void NewTaskForm::trigger(QString task) {
    QWidget *newTaskWidget = new QWidget(parentWidget);

    QVBoxLayout *newTaskLayout = new QVBoxLayout(newTaskWidget);

    QLabel *label = new QLabel("Add time to task:\n"+task);
    newTaskLayout->addWidget(label);

    taskName = new QLineEdit();
    newTaskLayout->addWidget(taskName);

    labels = new QLineEdit();
    newTaskLayout->addWidget(labels);

    submit = new QPushButton("Submit");
    connect( submit, &QPushButton::clicked, this, &NewTaskForm::submitRequest );
    newTaskLayout->addWidget(submit);

    cancel = new QPushButton(tr("Cancel"));
    connect( cancel, &QPushButton::clicked, this, &NewTaskForm::close );
    newTaskLayout->addWidget(cancel);

    newTaskWidget->setLayout(newTaskLayout);

    newTaskWidget->show();
    parentWidget->setSizePolicy(QSizePolicy::Expanding,QSizePolicy::Expanding);
}

void NewTaskForm::close() {
    taskName->deleteLater();
    labels->deleteLater();
    cancel->deleteLater();
    submit->deleteLater();
    parentWidget->childAt(0,0)->deleteLater();
    parentWidget->setSizePolicy(QSizePolicy::Minimum,QSizePolicy::Minimum);
}

void NewTaskForm::submitRequest() {
    BackendHandler backend;
    QString name = taskName->text().replace(" ","_");
    Result<QString> response = backend.newTask(taskName->text(),labels->text());
    if(response.hasError)
        statusDisplay->update(response.err());
    else
        statusDisplay->update(response.ok());

    close();

    reports->refresh();
}
