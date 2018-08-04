#ifndef PROJECTBOOKING_H
#define PROJECTBOOKING_H
#include <QWidget>
#include <QDate>
#include <QMenuBar>
#include <QMenu>
#include "taskmenu.h"

namespace Ui {
class ProjectBooking;
}

class ProjectBooking : public QWidget
{
    Q_OBJECT

public:
    explicit ProjectBooking(QWidget *parent = 0);
    ~ProjectBooking();
    #define VERSION = "010";

private slots:
    void on_ActiveProject_customContextMenuRequested();
    void on_MonthReportView_customContextMenuRequested();
    void on_StandardReportView_customContextMenuRequested();

private:
    Ui::ProjectBooking *ui;
    Reports *reports;
    StatusDisplay *statusDisplay;
};

#endif // PROJECTBOOKING_H
