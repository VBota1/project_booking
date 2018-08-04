#include "addtimeform.h"

AddRemoveTimeForm::AddRemoveTimeForm(Reports *referencedReports, StatusDisplay *status,QWidget *wparent) : QWidget(wparent)
{
    parentWidget=wparent;
    reports = referencedReports;
    statusDisplay = status;
}

void AddRemoveTimeForm::trigger(QString task) {
    QWidget *addTimeWidget = new QWidget(parentWidget);

    QVBoxLayout *addTimeLayout = new QVBoxLayout(addTimeWidget);

    QLabel *label = new QLabel("Add time to task:\n"+task);
    addTimeLayout->addWidget(label);

    inputData = new QDateTimeEdit();
    inputData->setDate(QDate::currentDate());
    addTimeLayout->addWidget(inputData);

    submitAdd = new QPushButton("Add time");
    submitAddMapper = new QSignalMapper(this);
    connect( submitAdd, SIGNAL(clicked()), submitAddMapper, SLOT(map()) );
    submitAddMapper->setMapping(submitAdd,task);
    connect(submitAddMapper, SIGNAL(mapped(QString)), this, SLOT(submitAddRequest(QString)));
    addTimeLayout->addWidget(submitAdd);

    submitRemove = new QPushButton("Remove time");
    submitRemoveMapper = new QSignalMapper(this);
    connect( submitRemove, SIGNAL(clicked()), submitRemoveMapper, SLOT(map()) );
    submitRemoveMapper->setMapping(submitRemove,task);
    connect(submitRemoveMapper, SIGNAL(mapped(QString)), this, SLOT(submitRemoveRequest(QString)));
    addTimeLayout->addWidget(submitRemove);

    cancel = new QPushButton(tr("Cancel"));
    connect( cancel, &QPushButton::clicked, this, &AddRemoveTimeForm::close );
    addTimeLayout->addWidget(cancel);

    addTimeWidget->setLayout(addTimeLayout);

    addTimeWidget->show();
    parentWidget->setSizePolicy(QSizePolicy::Expanding,QSizePolicy::Expanding);
}

void AddRemoveTimeForm::close() {
    inputData->deleteLater();
    cancel->deleteLater();
    submitAdd->deleteLater();
    submitAddMapper->deleteLater();
    parentWidget->childAt(0,0)->deleteLater();
    parentWidget->setSizePolicy(QSizePolicy::Minimum,QSizePolicy::Minimum);
}

void AddRemoveTimeForm::submitAddRequest(QString task) {
    BackendHandler backend;
    Result<QString> response = backend.addTime(task,inputData->dateTime());
    if(response.hasError)
        statusDisplay->update(response.err());
    else
        statusDisplay->update(response.ok());

    close();

    reports->refresh();
}

void AddRemoveTimeForm::submitRemoveRequest(QString task) {
    BackendHandler backend;
    Result<QString> response = backend.removeTime(task,inputData->dateTime());
    if(response.hasError)
        statusDisplay->update(response.err());
    else
        statusDisplay->update(response.ok());

    close();

    reports->refresh();
}
