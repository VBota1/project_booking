#ifndef PROJECTBOOKING_GUI_H
#define PROJECTBOOKING_GUI_H

#include <QMainWindow>

namespace Ui {
class ProjectBooking_gui;
}

class ProjectBooking_gui : public QMainWindow
{
    Q_OBJECT

public:
    explicit ProjectBooking_gui(QWidget *parent = 0);
    ~ProjectBooking_gui();

private:
    Ui::ProjectBooking_gui *ui;
};

#endif // PROJECTBOOKING_GUI_H
