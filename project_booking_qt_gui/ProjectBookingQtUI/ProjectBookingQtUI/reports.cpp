#include "reports.h"

Reports::Reports(QTreeWidget *month,QListWidget *standard,QListWidget *project,QLabel *current,StatusDisplay *statusWidget) {

    monthReport=month;
    standardReport=standard;
    projectReport=project;
    currentProject=current;
    statusDisplay = statusWidget;
}

void Reports::updateMonthReport(BackendHandler backend, int month){
    monthReport->clear();

    Result< QList<Data> > report = backend.reportForMonth(month);

    if (report.hasError)
    {
        statusDisplay->update(report.err());
        return;
    }

    Data day;
    foreach (day,report.ok()) {
        QTreeWidgetItem *dayItem = new QTreeWidgetItem(monthReport);
        monthReport->addTopLevelItem(dayItem);
        dayItem->setText(0,day.date);

        TaskDay task;
        foreach (task,day.tasks) {
            QTreeWidgetItem *taskItem = new QTreeWidgetItem(dayItem);
            taskItem->setText(0,task.task);

            QTreeWidgetItem *timeItem = new QTreeWidgetItem(taskItem);
            timeItem->setText(0,"time_spent: " + task.time_spent);

            QTreeWidgetItem *labelCollectionItem = new QTreeWidgetItem(taskItem);
            labelCollectionItem->setText(0,"labels");

            QString label;
            foreach (label, task.labels) {
                QTreeWidgetItem *labelItem = new QTreeWidgetItem(labelCollectionItem);
                labelItem->setText(0,   label);
            }
        }
    }
}

void Reports::updateStandardReport(BackendHandler backend) {
    standardReport->clear();

    Result< QList<TaskReport> > report = backend.standardReport();

    if (report.hasError)
    {
        statusDisplay->update(report.err());
        return;
    }

    updateCurrentProject("None");

    TaskReport task;
    foreach (task,report.ok()){
        QString taskText = "name: " + task.name.leftJustified(30,' ',true)
                            + "\t time: " + task.time_spent + "\t labels:";
        QString label;
        foreach(label,task.labels)
            taskText.append(" "+label);

        if(task.clock_in_timestamp!="None")
        {
            taskText.append("\t clockedIn: "+task.clock_in_timestamp);
            updateCurrentProject(task);
        }

        standardReport->addItem(taskText);
    }

}

void Reports::updateCurrentProject(TaskReport task)
{
    QString taskText = task.name + "\t labels:";
    QString label;
    foreach(label,task.labels)
        taskText.append(" "+label);
    updateCurrentProject(taskText);
}

void Reports::updateCurrentProject(QString task)
{
    currentProject->setText("Active Project: "+task);
    currentProject->update();
}

void Reports::updateProjectReport(BackendHandler backend) {
    projectReport->clear();

    Result< QList<LabelReport> > report = backend.projectLabelReport();

    if (report.hasError)
    {
        statusDisplay->update(report.err());
        return;
    }

    LabelReport project;
    foreach(project,report.ok())
    {
        QString projectText = "project: " + project.label.leftJustified(30,' ',true)
                            + "\t time: " + project.time_spent;
        projectReport->addItem(projectText);
    }
}

void Reports::refresh(){
    BackendHandler backend;
    updateMonthReport(backend, QDate::currentDate().month());
    updateStandardReport(backend);
    updateProjectReport(backend);
}

