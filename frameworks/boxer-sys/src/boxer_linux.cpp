#include "boxer.h"
#include <gtk/gtk.h>

namespace
{
   GtkMessageType getMessageType(BoxerStyle style)
   {
      switch (style)
      {
      case InfoStyle:
         return GTK_MESSAGE_INFO;
      case WarningStyle:
         return GTK_MESSAGE_WARNING;
      case ErrorStyle:
         return GTK_MESSAGE_ERROR;
      case QuestionStyle:
         return GTK_MESSAGE_QUESTION;
      default:
         return GTK_MESSAGE_INFO;
      }
   }

   GtkButtonsType getButtonsType(BoxerButtons buttons)
   {
      switch (buttons)
      {
      case OKButtons:
         return GTK_BUTTONS_OK;
      case OKCancelButtons:
         return GTK_BUTTONS_OK_CANCEL;
      case YesNoButtons:
         return GTK_BUTTONS_YES_NO;
     case QuitButtons:
         return GTK_BUTTONS_CLOSE;
      default:
         return GTK_BUTTONS_OK;
      }
   }

   BoxerSelection getSelection(gint response)
   {
      switch (response)
      {
      case GTK_RESPONSE_OK:
         return OKSelection;
      case GTK_RESPONSE_CANCEL:
         return CancelSelection;
      case GTK_RESPONSE_YES:
         return YesSelection;
      case GTK_RESPONSE_NO:
         return NoSelection;
      case GTK_RESPONSE_CLOSE:
         return QuitSelection;
      default:
         return NoneSelection;
      }
   }
} // namespace

extern "C"
BoxerSelection boxer_show(const char* message, const char* title, BoxerStyle style, BoxerButtons buttons)
{
   if (!gtk_init_check(0, nullptr))
   {
      return ErrorSelection;
   }

   // Create a parent window to stop gtk_dialog_run from complaining
   GtkWidget* parent = gtk_window_new(GTK_WINDOW_TOPLEVEL);

   GtkWidget* dialog = gtk_message_dialog_new(GTK_WINDOW(parent),
                                              GTK_DIALOG_MODAL,
                                              getMessageType(style),
                                              getButtonsType(buttons),
                                              "%s",
                                              message);
   gtk_window_set_title(GTK_WINDOW(dialog), title);
   Selection selection = getSelection(gtk_dialog_run(GTK_DIALOG(dialog)));

   gtk_widget_destroy(GTK_WIDGET(dialog));
   gtk_widget_destroy(GTK_WIDGET(parent));
   while (g_main_context_iteration(nullptr, false));

   return selection;
}

