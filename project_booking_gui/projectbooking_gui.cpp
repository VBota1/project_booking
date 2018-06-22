#include "projectbooking_gui.h"
#include "ui_projectbooking_gui.h"

ProjectBooking_gui::ProjectBooking_gui(QWidget *parent) :
    QMainWindow(parent),
    ui(new Ui::ProjectBooking_gui)
{
    ui->setupUi(this);
}

ProjectBooking_gui::~ProjectBooking_gui()
{
    delete ui;
}
