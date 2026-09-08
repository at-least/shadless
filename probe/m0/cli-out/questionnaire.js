"use client";
import * as React from "react";
import { Questionnaire as QuestionnairePrimitive } from "@shadcn/react/questionnaire";
import { cn } from "@/registry/bases/radix/lib/utils";
import { buttonVariants } from "@/registry/bases/radix/ui/button";
import { IconPlaceholder } from "@/app/(create)/components/icon-placeholder";
function Questionnaire({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    QuestionnairePrimitive.Root,
    {
      "data-slot": "questionnaire",
      className: cn("gap-4 flex w-full min-w-0 flex-col", className),
      ...props
    }
  );
}
function QuestionnaireProgress({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    QuestionnairePrimitive.Progress,
    {
      "data-slot": "questionnaire-progress",
      className: cn(
        "text-xs min-h-[1lh] w-fit min-w-[14ch] font-medium text-muted-foreground tabular-nums",
        className
      ),
      ...props
    }
  );
}
function QuestionnaireItem({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    QuestionnairePrimitive.Item,
    {
      "data-slot": "questionnaire-item",
      className: cn(
        "flex flex-col gap-4 min-w-0 border-0 p-0 outline-none",
        className
      ),
      ...props
    }
  );
}
function QuestionnaireTitle({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    QuestionnairePrimitive.Title,
    {
      "data-slot": "questionnaire-title",
      className: cn(
        "text-base leading-snug font-medium [&:not(:has(~[data-slot=questionnaire-description]))]:mb-4 cn-font-heading text-pretty",
        className
      ),
      ...props
    }
  );
}
function QuestionnaireDescription({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    QuestionnairePrimitive.Description,
    {
      "data-slot": "questionnaire-description",
      className: cn(
        "text-sm text-pretty text-muted-foreground",
        className
      ),
      ...props
    }
  );
}
function QuestionnaireChoices({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    QuestionnairePrimitive.Choices,
    {
      "data-slot": "questionnaire-choices",
      className: cn(
        "gap-2 group/questionnaire-choices grid min-w-0",
        className
      ),
      ...props
    }
  );
}
function QuestionnaireChoice({
  children,
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    QuestionnairePrimitive.Choice,
    {
      "data-slot": "questionnaire-choice",
      className: cn(
        "border-input dark:bg-input/20 hover:bg-muted/50 data-checked:border-primary/40 data-checked:bg-muted dark:data-checked:bg-muted data-invalid:border-destructive has-[>input:focus-visible]:border-ring has-[>input:focus-visible]:ring-ring/50 gap-2.5 rounded-lg border bg-transparent px-3 py-2.5 text-sm has-[>input:focus-visible]:ring-3 group/questionnaire-choice relative flex min-h-11 cursor-pointer items-start text-start transition-colors outline-none select-none",
        "data-disabled:pointer-events-none data-disabled:cursor-not-allowed data-disabled:opacity-50",
        className
      ),
      ...props
    },
    /* @__PURE__ */ React.createElement(
      QuestionnairePrimitive.ChoiceInput,
      {
        "data-slot": "questionnaire-choice-input",
        className: "absolute inset-0 z-10 size-full cursor-pointer opacity-0"
      }
    ),
    /* @__PURE__ */ React.createElement(
      "span",
      {
        "aria-hidden": "true",
        "data-slot": "questionnaire-choice-indicator",
        className: "border-input dark:bg-input/30 group-data-checked/questionnaire-choice:bg-primary dark:group-data-checked/questionnaire-choice:bg-primary group-data-checked/questionnaire-choice:text-primary-foreground group-data-checked/questionnaire-choice:border-primary size-4 translate-y-[--spacing(0.45)] group-has-data-[slot=questionnaire-choice-description]/questionnaire-choice:translate-y-0.5 rounded-[4px] pointer-events-none relative flex shrink-0 items-center justify-center border group-data-[type=radio]/questionnaire-choice:rounded-full"
      },
      /* @__PURE__ */ React.createElement(
        "span",
        {
          "data-slot": "questionnaire-choice-indicator-dot",
          className: "bg-primary-foreground size-2 hidden rounded-full group-data-[type=checkbox]/questionnaire-choice:hidden group-data-checked/questionnaire-choice:block"
        }
      ),
      /* @__PURE__ */ React.createElement(
        IconPlaceholder,
        {
          "data-slot": "questionnaire-choice-indicator-check",
          className: "size-3.5 hidden group-data-[type=radio]/questionnaire-choice:hidden group-data-checked/questionnaire-choice:block",
          lucide: "CheckIcon",
          tabler: "IconCheck",
          hugeicons: "Tick02Icon",
          phosphor: "CheckIcon",
          remixicon: "RiCheckLine"
        }
      )
    ),
    /* @__PURE__ */ React.createElement(
      QuestionnairePrimitive.ChoiceLabel,
      {
        "data-slot": "questionnaire-choice-label",
        className: "gap-0.5 flex min-w-0 flex-1 flex-col leading-snug"
      },
      children
    ),
    /* @__PURE__ */ React.createElement(
      QuestionnairePrimitive.ChoiceShortcut,
      {
        "data-slot": "questionnaire-choice-shortcut",
        className: "border-input bg-background text-muted-foreground size-5 translate-y-[--spacing(0.45)] group-has-data-[slot=questionnaire-choice-description]/questionnaire-choice:translate-y-0.5 items-center justify-center rounded-md border font-mono text-[0.625rem] font-medium leading-none pointer-events-none ms-auto hidden shrink-0 group-data-[shortcut]/questionnaire-choice:inline-flex"
      }
    )
  );
}
function QuestionnaireChoiceDescription({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    "span",
    {
      "data-slot": "questionnaire-choice-description",
      className: cn("text-muted-foreground", className),
      ...props
    }
  );
}
function QuestionnaireInput({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    "div",
    {
      "data-slot": "questionnaire-input-wrapper",
      className: "w-full group/questionnaire-input relative min-w-0"
    },
    /* @__PURE__ */ React.createElement(
      QuestionnairePrimitive.Input,
      {
        "data-slot": "questionnaire-input",
        className: cn(
          "dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base focus-visible:ring-3 aria-invalid:ring-3 md:text-sm min-h-11 w-full min-w-0 transition-[color,box-shadow,background-color] outline-none disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 sm:min-h-0",
          "selection:bg-primary selection:text-primary-foreground placeholder:text-muted-foreground",
          className
        ),
        ...props
      }
    )
  );
}
function QuestionnaireError({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    QuestionnairePrimitive.Error,
    {
      "data-slot": "questionnaire-error",
      className: cn("mt-2 text-sm text-destructive", className),
      ...props
    }
  );
}
function QuestionnaireActions({
  className,
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    "div",
    {
      "data-slot": "questionnaire-actions",
      className: cn(
        "gap-2 sm:min-h-8 grid min-h-11 w-full grid-cols-[minmax(0,1fr)_auto_auto] items-center",
        className
      ),
      ...props
    }
  );
}
function QuestionnairePrevious({
  children,
  className,
  size = "default",
  variant = "outline",
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    QuestionnairePrimitive.Previous,
    {
      "data-slot": "questionnaire-previous",
      "data-size": size,
      "data-variant": variant,
      className: cn(
        buttonVariants({ size, variant }),
        "col-start-1 row-start-1 min-h-11 justify-self-start sm:min-h-0",
        className
      ),
      ...props
    },
    children ?? "Previous"
  );
}
function QuestionnaireSkip({
  children,
  className,
  size = "default",
  variant = "outline",
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    QuestionnairePrimitive.Skip,
    {
      "data-slot": "questionnaire-skip",
      "data-size": size,
      "data-variant": variant,
      className: cn(
        buttonVariants({ size, variant }),
        "col-start-2 row-start-1 min-h-11 justify-self-end sm:min-h-0",
        className
      ),
      ...props
    },
    children ?? "Skip"
  );
}
function QuestionnaireNext({
  children,
  className,
  size = "default",
  variant = "default",
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    QuestionnairePrimitive.Next,
    {
      "data-slot": "questionnaire-next",
      "data-size": size,
      "data-variant": variant,
      className: cn(
        buttonVariants({ size, variant }),
        "col-start-3 row-start-1 min-h-11 justify-self-end sm:min-h-0",
        className
      ),
      ...props
    },
    children ?? "Next"
  );
}
function QuestionnaireSubmit({
  children,
  className,
  size = "default",
  variant = "default",
  ...props
}) {
  return /* @__PURE__ */ React.createElement(
    QuestionnairePrimitive.Submit,
    {
      "data-slot": "questionnaire-submit",
      "data-size": size,
      "data-variant": variant,
      className: cn(
        buttonVariants({ size, variant }),
        "col-start-3 row-start-1 min-h-11 justify-self-end sm:min-h-0",
        className
      ),
      ...props
    },
    children ?? "Submit"
  );
}
export {
  Questionnaire,
  QuestionnaireActions,
  QuestionnaireChoice,
  QuestionnaireChoiceDescription,
  QuestionnaireChoices,
  QuestionnaireDescription,
  QuestionnaireError,
  QuestionnaireInput,
  QuestionnaireItem,
  QuestionnaireNext,
  QuestionnairePrevious,
  QuestionnaireProgress,
  QuestionnaireSkip,
  QuestionnaireSubmit,
  QuestionnaireTitle
};
