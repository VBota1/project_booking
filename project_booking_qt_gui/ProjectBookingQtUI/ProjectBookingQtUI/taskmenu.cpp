#include "taskmenu.h"

TaskMenu::TaskMenu(Reports *referencedReports, StatusDisplay *status, Ui::ProjectBooking *uiParent, QWidget *parent, QString task) : QWidget(parent)
{
    reports = referencedReports;
    statusDisplay = status;
    actionsMenu = uiParent->ActionsMenu;
    parentUi = uiParent;
    parentWidget = parent;
    trigger(task);
}

void TaskMenu::trigger(QString task) {
    QAction *clockOutAction = new QAction(tr("Clock Out"),this);
    QSignalMapper clockOutMapper;
    connect( clockOutAction, SIGNAL(triggered()), &clockOutMapper, SLOT(map()));
    clockOutMapper.setMapping(clockOutAction,task);
    connect(&clockOutMapper, SIGNAL(mapped(QString)), this, SLOT(clockOut(QString)) );

    QAction *clockInAction = new QAction(tr("Clock In"),this);
    QSignalMapper clockInMapper;
    connect( clockInAction, SIGNAL(triggered()), &clockInMapper, SLOT(map()));
    clockInMapper.setMapping(clockInAction,task);
    connect(&clockInMapper, SIGNAL(mapped(QString)), this, SLOT(clockIn(QString)) );

    addRemoveTimeForm = new AddRemoveTimeForm(reports,statusDisplay,actionsMenu);
    QAction *addTimeAction = new QAction(tr("Add/Remove Time"),this);
    QSignalMapper addTimeMapper;
    connect( addTimeAction, SIGNAL(triggered()), &addTimeMapper, SLOT(map()) );
    addTimeMapper.setMapping(addTimeAction,task);
    connect(&addTimeMapper, SIGNAL(mapped(QString)), addRemoveTimeForm, SLOT(trigger(QString)) );

    newTaskForm = new NewTaskForm(reports,statusDisplay,actionsMenu);
    QAction *newTaskAction = new QAction(tr("New Task"),this);
    QSignalMapper newTaskMapper;
    connect( newTaskAction, SIGNAL(triggered()), &newTaskMapper, SLOT(map()) );
    newTaskMapper.setMapping(newTaskAction,task);
    connect(&newTaskMapper, SIGNAL(mapped(QString)), newTaskForm, SLOT(trigger(QString)) );

    QAction *removeTaskAction = new QAction(tr("Delete Task"),this);
    QSignalMapper removeTaskMapper;
    connect( removeTaskAction, SIGNAL(triggered()), &removeTaskMapper, SLOT(map()));
    removeTaskMapper.setMapping(removeTaskAction,task);
    connect(&removeTaskMapper, SIGNAL(mapped(QString)), this, SLOT(remove(QString)) );

    helpForm = new HelpForm(reports,statusDisplay,actionsMenu);
    QAction *helpAction = new QAction(tr("Help"),this);
    connect( helpAction, SIGNAL(triggered()), helpForm,  SLOT(trigger()) );

    licenseForm = new LicenseForm(reports,statusDisplay,actionsMenu);
    QAction *licenseAction = new QAction(tr("License"),this);
    connect( licenseAction, SIGNAL(triggered()), licenseForm,  SLOT(trigger()) );

    QAction *foldAction = new QAction(tr("Fold"),this);
    connect( foldAction, &QAction::triggered, this,  &TaskMenu::fold );

    QAction *unfoldAction = new QAction(tr("Unfold"),this);
    connect( unfoldAction, &QAction::triggered, this,  &TaskMenu::unfold );

    QMenu activeProjectContextMenu;
    activeProjectContextMenu.addAction(newTaskAction);
    activeProjectContextMenu.addAction(removeTaskAction);
    activeProjectContextMenu.addAction(clockInAction);
    activeProjectContextMenu.addAction(clockOutAction);
    activeProjectContextMenu.addAction(addTimeAction);
    activeProjectContextMenu.addAction(helpAction);
    activeProjectContextMenu.addAction(licenseAction);
    activeProjectContextMenu.addAction(foldAction);
    activeProjectContextMenu.addAction(unfoldAction);
    activeProjectContextMenu.exec(QCursor::pos());
}

void TaskMenu::clockOut(QString taskName) {
    BackendHandler backend;
    Result<QString> response = backend.clockOut(taskName);
    if(response.hasError)
        statusDisplay->update(response.err());
    else
        statusDisplay->update(response.ok());

    reports->refresh();
}

void TaskMenu::clockIn(QString taskName) {
    BackendHandler backend;
    Result<QString> response = backend.clockIn(taskName);
    if(response.hasError)
        statusDisplay->update(response.err());
    else
        statusDisplay->update(response.ok());

    reports->refresh();
}

void TaskMenu::remove(QString taskName) {
    BackendHandler backend;
    Result<QString> response = backend.remove(taskName);
    if(response.hasError)
        statusDisplay->update(response.err());
    else
        statusDisplay->update(response.ok());

    reports->refresh();
}

void TaskMenu::fold() {
    parentUi->Status->hide();
    parentUi->MainWindow->hide();
    parentUi->ActionsMenu->hide();
    parentWidget->setMaximumHeight(20);
}

void TaskMenu::unfold() {
    parentWidget->setMaximumHeight(16777215);
    parentWidget->resize(963,400);
    parentUi->Status->show();
    parentUi->MainWindow->show();
    parentUi->ActionsMenu->show();
}
