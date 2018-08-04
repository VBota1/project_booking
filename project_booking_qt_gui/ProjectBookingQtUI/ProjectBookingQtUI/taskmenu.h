#ifndef TASKMENU_H
#define TASKMENU_H
#include <QMenu>
#include <QMenuBar>
#include <QSignalMapper>
#include "reports.h"
#include "addtimeform.h"
#include "newtaskform.h"
#include "helpform.h"
#include "licenseform.h"

class TaskMenu : public QWidget
{
    Q_OBJECT

public:
    explicit TaskMenu(Reports *referencedReports, StatusDisplay *status, Ui::ProjectBooking *uiParent, QWidget *parent, QString task);

private slots:
    void trigger(QString task);
    void clockIn(QString task);
    void clockOut(QString task);
    void remove(QString task);

private:
    Reports *reports;
    StatusDisplay *statusDisplay;
    AddRemoveTimeForm *addRemoveTimeForm;
    NewTaskForm *newTaskForm;
    HelpForm *helpForm;
    LicenseForm *licenseForm;
    QWidget *actionsMenu;
    Ui::ProjectBooking *parentUi;
    QWidget *parentWidget;
    void fold();
    void unfold();
};

#endif // TASKMENU_H
