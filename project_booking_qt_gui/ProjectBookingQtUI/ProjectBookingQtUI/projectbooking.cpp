#include "projectbooking.h"

ProjectBooking::ProjectBooking(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::ProjectBooking)
{
    ui->setupUi(this);
    ui->MainWindow->setCurrentWidget(ui->MonthReport);

    statusDisplay = new StatusDisplay(ui->StatusList);

    reports = new Reports(ui->MonthReportView,ui->StandardReportView,ui->ProjectReportView,ui->ActiveProject,statusDisplay);
    reports->refresh();
}

ProjectBooking::~ProjectBooking()
{
    delete ui;
}

void ProjectBooking::on_ActiveProject_customContextMenuRequested()
{
    QString taskName = ui->ActiveProject->text();
    taskName.remove("Active Project: ");
    new TaskMenu(reports,statusDisplay,ui,this,taskName);
}

void ProjectBooking::on_MonthReportView_customContextMenuRequested()
{
    if(ui->MonthReportView->selectedItems().isEmpty())
        return;

    QTreeWidgetItem *item = ui->MonthReportView->selectedItems().first();

    if(!item->parent())
        return;

    if(item->parent()->parent())
        return;

    new TaskMenu(reports,statusDisplay,ui,this,item->text(0));
}

void ProjectBooking::on_StandardReportView_customContextMenuRequested()
{
    if(ui->StandardReportView->selectedItems().isEmpty())
        return;

    QListWidgetItem *item = ui->StandardReportView->selectedItems().first();
    QString text = item->text();
    QString startLabel = "name: ";
    int start = text.indexOf(startLabel) + startLabel.length();
    int end = text.indexOf("\t time: ");
    text = text.mid(start,(end-start));
    text = text.simplified();
    new TaskMenu(reports,statusDisplay,ui,this,text);
}
/*
void ProjectBooking::fold() {
    ui->Status->hide();
    ui->MainWindow->hide();
    ui->ActionsMenu->hide();
    this->setMaximumHeight(20);
}

void ProjectBooking::unfold() {
    this->setMaximumHeight(16777215);
    ui->Status->show();
    ui->MainWindow->show();
    ui->ActionsMenu->show();
}
*/

