#include "boxer.h"
#import <Cocoa/Cocoa.h>

namespace
{
#if defined(MAC_OS_X_VERSION_10_12) && MAC_OS_X_VERSION_MIN_REQUIRED >= MAC_OS_X_VERSION_10_12
   const NSAlertStyle kInformationalStyle = NSAlertStyleInformational;
   const NSAlertStyle kWarningStyle = NSAlertStyleWarning;
   const NSAlertStyle kCriticalStyle = NSAlertStyleCritical;
#else
   const NSAlertStyle kInformationalStyle = NSInformationalAlertStyle;
   const NSAlertStyle kWarningStyle = NSWarningAlertStyle;
   const NSAlertStyle kCriticalStyle = NSCriticalAlertStyle;
#endif

#if defined(MAC_OS_X_VERSION_10_9) && MAC_OS_X_VERSION_MIN_REQUIRED >= MAC_OS_X_VERSION_10_9
   using ModalResponse = NSModalResponse;
#elif defined(MAC_OS_X_VERSION_10_5) && MAC_OS_X_VERSION_MIN_REQUIRED >= MAC_OS_X_VERSION_10_5
   using ModalResponse = NSInteger;
#else
   using ModalResponse = int;
#endif

   NSString* const kOkStr = @"OK";
   NSString* const kCancelStr = @"Cancel";
   NSString* const kYesStr = @"Yes";
   NSString* const kNoStr = @"No";
   NSString* const kQuitStr = @"Quit";

   NSAlertStyle getAlertStyle(Style style)
   {
      switch (style)
      {
      case InfoStyle:
         return kInformationalStyle;
      case WarningStyle:
         return kWarningStyle;
      case ErrorStyle:
         return kCriticalStyle;
      case QuestionStyle:
         return kWarningStyle;
      default:
         return kInformationalStyle;
      }
   }

   void setButtons(NSAlert* alert, BoxerButtons buttons)
   {
      switch (buttons)
      {
      case OKButtons:
         [alert addButtonWithTitle:kOkStr];
         break;
      case OKCancelButtons:
         [alert addButtonWithTitle:kOkStr];
         [alert addButtonWithTitle:kCancelStr];
         break;
      case YesNoButtons:
         [alert addButtonWithTitle:kYesStr];
         [alert addButtonWithTitle:kNoStr];
         break;
     case QuitButtons:
         [alert addButtonWithTitle:kQuitStr];
       break;
      default:
         [alert addButtonWithTitle:kOkStr];
      }
   }

   BoxerSelection getSelection(ModalResponse index, BoxerButtons buttons)
   {
      switch (buttons)
      {
      case OKButtons:
         return index == NSAlertFirstButtonReturn ? OKSelection : NoneSelection;
      case OKCancelButtons:
         if (index == NSAlertFirstButtonReturn)
         {
            return OKSelection;
         }
         else if (index == NSAlertSecondButtonReturn)
         {
            return CancelSelection;
         }
         else
         {
            return NoneSelection;
         }
      case YesNoButtons:
         if (index == NSAlertFirstButtonReturn)
         {
            return YesSelection;
         }
         else if (index == NSAlertSecondButtonReturn)
         {
            return NoSelection;
         }
         else
         {
            return NoneSelection;
         }
      case QuitButtons:
         return index == NSAlertFirstButtonReturn ? QuitSelection : NoneSelection;
      default:
         return NoneSelection;
      }
   }
} // namespace

extern "C"
BoxerSelection boxer_show(const char* message, const char* title, BoxerStyle style, BoxerButtons buttons)
{
   NSAlert* alert = [[NSAlert alloc] init];

   [alert setMessageText:[NSString stringWithUTF8String:title]];
   [alert setInformativeText:[NSString stringWithUTF8String:message]];

   [alert setAlertStyle:getAlertStyle(style)];
   setButtons(alert, buttons);

   // Force the alert to appear on top of any other windows
   [[alert window] setLevel:NSModalPanelWindowLevel];

   BoxerSelection selection = getSelection([alert runModal], buttons);
   [alert release];

   return selection;
}

