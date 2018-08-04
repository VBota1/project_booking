#ifndef REPORTSHANDLER_H
#define REPORTSHANDLER_H
#include "backendhandler.h"
#include "statusdisplay.h"

class Reports
{
private:
    QTreeWidget *monthReport;
    QListWidget *standardReport;
    QListWidget *projectReport;
    QLabel *currentProject;
    StatusDisplay *statusDisplay;
    void updateMonthReport(BackendHandler backend, int month);
    void updateStandardReport(BackendHandler backend);
    void updateProjectReport(BackendHandler backend);
    void updateCurrentProject(TaskReport task);
    void updateCurrentProject(QString task);
public:
    Reports(QTreeWidget *,QListWidget *,QListWidget *,QLabel *,StatusDisplay *);
    void refresh();
};

#endif // REPORTSHANDLER_H
