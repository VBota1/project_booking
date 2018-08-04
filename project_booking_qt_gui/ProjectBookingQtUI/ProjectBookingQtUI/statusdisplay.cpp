#include "statusdisplay.h"

StatusDisplay::StatusDisplay(QListWidget *widget){
    statusWidget = widget;
}

void StatusDisplay::update(QString message) {
    message = message.simplified();
    statusWidget->addItem(message);
    statusWidget->scrollToBottom();
}

