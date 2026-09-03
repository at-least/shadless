---
title: "Field"
description: "Combine labels, controls, and help text to compose accessible form fields and grouped inputs."
---

# Field

Combine labels, controls, and help text to compose accessible form fields and grouped inputs.

::::demo field-demo
<iframe class="demo" src="/demos/field-demo.html" title="field-demo" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/field-demo.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [field-demo.html]
<div class="w-full max-w-md">
  <form>
    <div
      data-slot="field-group"
      class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col"
    >
      <fieldset
        data-slot="field-set"
        class="gap-4 has-[&gt;[data-slot=checkbox-group]]:gap-3 has-[&gt;[data-slot=radio-group]]:gap-3 flex flex-col"
      >
        <legend
          data-slot="field-legend"
          data-variant="legend"
          class="mb-1.5 font-medium data-[variant=label]:text-sm data-[variant=legend]:text-base"
        >
          Payment Method
        </legend>
        <p
          data-slot="field-description"
          class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
        >
          All transactions are secure and encrypted
        </p>
        <div
          data-slot="field-group"
          class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col"
        >
          <div
            role="group"
            data-slot="field"
            data-orientation="vertical"
            class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
          >
            <label
              data-slot="field-label"
              class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
              for="checkout-7j9-card-name-43j"
              >Name on Card</label
            ><input
              data-slot="input"
              class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
              id="checkout-7j9-card-name-43j"
              placeholder="Evil Rabbit"
              required=""
            />
          </div>
          <div
            role="group"
            data-slot="field"
            data-orientation="vertical"
            class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
          >
            <label
              data-slot="field-label"
              class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
              for="checkout-7j9-card-number-uw1"
              >Card Number</label
            ><input
              data-slot="input"
              class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
              id="checkout-7j9-card-number-uw1"
              placeholder="1234 5678 9012 3456"
              required=""
            />
            <p
              data-slot="field-description"
              class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
            >
              Enter your 16-digit card number
            </p>
          </div>
          <div class="grid grid-cols-3 gap-4">
            <div
              role="group"
              data-slot="field"
              data-orientation="vertical"
              class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
            >
              <label
                data-slot="field-label"
                class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
                for="checkout-exp-month-ts6"
                >Month</label
              ><button
                type="button"
                role="combobox"
                aria-controls="s0"
                aria-expanded="false"
                aria-autocomplete="none"
                dir="ltr"
                data-state="closed"
                data-placeholder=""
                data-slot="select-trigger"
                data-size="default"
                class="border-input data-placeholder:text-muted-foreground dark:bg-input/30 dark:hover:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 gap-1.5 rounded-lg border bg-transparent py-2 pr-2 pl-2.5 text-sm transition-colors select-none focus-visible:ring-3 aria-invalid:ring-3 data-[size=default]:h-8 data-[size=sm]:h-7 data-[size=sm]:rounded-[min(var(--radius-md),10px)] *:data-[slot=select-value]:gap-1.5 [&amp;_svg:not([class*='size-'])]:size-4 flex w-fit items-center justify-between whitespace-nowrap outline-none disabled:cursor-not-allowed disabled:opacity-50 *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex *:data-[slot=select-value]:items-center [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
                id="s0-trigger"
              >
                <span data-slot="select-value" style="pointer-events: none">MM</span
                ><svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="24"
                  height="24"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  class="lucide lucide-chevron-down text-muted-foreground size-4 pointer-events-none"
                >
                  <path d="m6 9 6 6 6-6"></path>
                </svg></button
              ><select
                tabindex="-1"
                style="
                  position: absolute;
                  border: 0px;
                  width: 1px;
                  height: 1px;
                  padding: 0px;
                  margin: -1px;
                  overflow: hidden;
                  clip: rect(0px, 0px, 0px, 0px);
                  white-space: nowrap;
                  overflow-wrap: normal;
                "
              >
                <option value="01">01</option>
                <option value="02">02</option>
                <option value="03">03</option>
                <option value="04">04</option>
                <option value="05">05</option>
                <option value="06">06</option>
                <option value="07">07</option>
                <option value="08">08</option>
                <option value="09">09</option>
                <option value="10">10</option>
                <option value="11">11</option>
                <option value="12">12</option>
              </select>
            </div>
            <div
              role="group"
              data-slot="field"
              data-orientation="vertical"
              class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
            >
              <label
                data-slot="field-label"
                class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
                for="checkout-7j9-exp-year-f59"
                >Year</label
              ><button
                type="button"
                role="combobox"
                aria-controls="s1"
                aria-expanded="false"
                aria-autocomplete="none"
                dir="ltr"
                data-state="closed"
                data-placeholder=""
                data-slot="select-trigger"
                data-size="default"
                class="border-input data-placeholder:text-muted-foreground dark:bg-input/30 dark:hover:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 gap-1.5 rounded-lg border bg-transparent py-2 pr-2 pl-2.5 text-sm transition-colors select-none focus-visible:ring-3 aria-invalid:ring-3 data-[size=default]:h-8 data-[size=sm]:h-7 data-[size=sm]:rounded-[min(var(--radius-md),10px)] *:data-[slot=select-value]:gap-1.5 [&amp;_svg:not([class*='size-'])]:size-4 flex w-fit items-center justify-between whitespace-nowrap outline-none disabled:cursor-not-allowed disabled:opacity-50 *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex *:data-[slot=select-value]:items-center [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
                id="s1-trigger"
              >
                <span data-slot="select-value" style="pointer-events: none">YYYY</span
                ><svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="24"
                  height="24"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  class="lucide lucide-chevron-down text-muted-foreground size-4 pointer-events-none"
                >
                  <path d="m6 9 6 6 6-6"></path>
                </svg></button
              ><select
                tabindex="-1"
                style="
                  position: absolute;
                  border: 0px;
                  width: 1px;
                  height: 1px;
                  padding: 0px;
                  margin: -1px;
                  overflow: hidden;
                  clip: rect(0px, 0px, 0px, 0px);
                  white-space: nowrap;
                  overflow-wrap: normal;
                "
              >
                <option value="2024">2024</option>
                <option value="2025">2025</option>
                <option value="2026">2026</option>
                <option value="2027">2027</option>
                <option value="2028">2028</option>
                <option value="2029">2029</option>
              </select>
            </div>
            <div
              role="group"
              data-slot="field"
              data-orientation="vertical"
              class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
            >
              <label
                data-slot="field-label"
                class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
                for="checkout-7j9-cvv"
                >CVV</label
              ><input
                data-slot="input"
                class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
                id="checkout-7j9-cvv"
                placeholder="123"
                required=""
              />
            </div>
          </div>
        </div>
      </fieldset>
      <div
        data-slot="field-separator"
        data-content="false"
        class="-my-2 h-5 text-sm group-data-[variant=outline]/field-group:-mb-2 relative"
      >
        <div
          data-orientation="horizontal"
          role="none"
          data-slot="separator"
          class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch absolute inset-0 top-1/2"
        ></div>
      </div>
      <fieldset
        data-slot="field-set"
        class="gap-4 has-[&gt;[data-slot=checkbox-group]]:gap-3 has-[&gt;[data-slot=radio-group]]:gap-3 flex flex-col"
      >
        <legend
          data-slot="field-legend"
          data-variant="legend"
          class="mb-1.5 font-medium data-[variant=label]:text-sm data-[variant=legend]:text-base"
        >
          Billing Address
        </legend>
        <p
          data-slot="field-description"
          class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
        >
          The billing address associated with your payment method
        </p>
        <div
          data-slot="field-group"
          class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col"
        >
          <div
            role="group"
            data-slot="field"
            data-orientation="horizontal"
            class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
          >
            <button
              type="button"
              role="checkbox"
              aria-checked="true"
              data-state="checked"
              value="on"
              data-slot="checkbox"
              class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
              id="checkout-7j9-same-as-shipping-wgm"
            >
              <span
                data-state="checked"
                data-slot="checkbox-indicator"
                class="[&amp;&gt;svg]:size-3.5 grid place-content-center text-current transition-none"
                style="pointer-events: none"
                ><svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="24"
                  height="24"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  class="lucide lucide-check"
                >
                  <path d="M20 6 9 17l-5-5"></path></svg
              ></span></button
            ><input
              tabindex="-1"
              type="checkbox"
              value="on"
              checked=""
              style="
                position: absolute;
                pointer-events: none;
                opacity: 0;
                margin: 0px;
                transform: translateX(-100%);
                width: 40px;
                height: 33px;
              "
            /><label
              data-slot="field-label"
              class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
              for="checkout-7j9-same-as-shipping-wgm"
              >Same as shipping address</label
            >
          </div>
        </div>
      </fieldset>
      <fieldset
        data-slot="field-set"
        class="gap-4 has-[&gt;[data-slot=checkbox-group]]:gap-3 has-[&gt;[data-slot=radio-group]]:gap-3 flex flex-col"
      >
        <div
          data-slot="field-group"
          class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col"
        >
          <div
            role="group"
            data-slot="field"
            data-orientation="vertical"
            class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
          >
            <label
              data-slot="field-label"
              class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
              for="checkout-7j9-optional-comments"
              >Comments</label
            ><textarea
              data-slot="textarea"
              class="border-input dark:bg-input/30 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 rounded-lg border bg-transparent px-2.5 py-2 text-base transition-colors focus-visible:ring-3 aria-invalid:ring-3 md:text-sm flex field-sizing-content min-h-16 w-full outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50 resize-none"
              id="checkout-7j9-optional-comments"
              placeholder="Add any additional comments"
            ></textarea>
          </div>
        </div>
      </fieldset>
      <div
        role="group"
        data-slot="field"
        data-orientation="horizontal"
        class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
      >
        <button
          data-slot="button"
          data-variant="default"
          data-size="default"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
          type="submit"
        >
          Submit</button
        ><button
          data-slot="button"
          data-variant="outline"
          data-size="default"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
          type="button"
        >
          Cancel
        </button>
      </div>
    </div>
  </form>
</div>
<template id="s0-tpl">
  <div
    role="listbox"
    id="s0"
    data-state="open"
    dir="ltr"
    data-slot="select-content"
    data-align-trigger="true"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent relative z-50 max-h-(--radix-select-content-available-height) origin-(--radix-select-content-transform-origin) overflow-x-hidden overflow-y-auto data-[align-trigger=true]:animate-none"
    tabindex="-1"
    style="
      box-sizing: border-box;
      max-height: 100%;
      display: flex;
      flex-direction: column;
      outline: none;
      pointer-events: auto;
    "
  >
    <style>
      [data-radix-select-viewport] {
        scrollbar-width: none;
        -ms-overflow-style: none;
        -webkit-overflow-scrolling: touch;
      }
      [data-radix-select-viewport]::-webkit-scrollbar {
        display: none;
      }
    </style>
    <div
      data-radix-select-viewport=""
      role="presentation"
      data-position="item-aligned"
      class="data-[position=popper]:h-(--radix-select-trigger-height) data-[position=popper]:w-full data-[position=popper]:min-w-(--radix-select-trigger-width)"
      style="position: relative; flex: 1 1 0%; overflow: hidden auto"
    >
      <div role="group" aria-labelledby="-trigger" data-slot="select-group" class="scroll-my-1 p-1">
        <div
          role="option"
          aria-labelledby="s0-e0"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
          data-highlighted=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e0">01</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e1"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e1">02</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e2"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e2">03</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e3"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e3">04</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e4"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e4">05</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e5"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e5">06</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e6"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e6">07</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e7"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e7">08</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e8"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e8">09</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e9"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e9">10</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e10"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e10">11</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e11"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e11">12</span>
        </div>
      </div>
    </div>
  </div>
</template>
<template id="s1-tpl">
  <div
    role="listbox"
    id="s1"
    data-state="open"
    dir="ltr"
    data-slot="select-content"
    data-align-trigger="true"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent relative z-50 max-h-(--radix-select-content-available-height) origin-(--radix-select-content-transform-origin) overflow-x-hidden overflow-y-auto data-[align-trigger=true]:animate-none"
    tabindex="-1"
    style="
      box-sizing: border-box;
      max-height: 100%;
      display: flex;
      flex-direction: column;
      outline: none;
      pointer-events: auto;
    "
  >
    <style>
      [data-radix-select-viewport] {
        scrollbar-width: none;
        -ms-overflow-style: none;
        -webkit-overflow-scrolling: touch;
      }
      [data-radix-select-viewport]::-webkit-scrollbar {
        display: none;
      }
    </style>
    <div
      data-radix-select-viewport=""
      role="presentation"
      data-position="item-aligned"
      class="data-[position=popper]:h-(--radix-select-trigger-height) data-[position=popper]:w-full data-[position=popper]:min-w-(--radix-select-trigger-width)"
      style="position: relative; flex: 1 1 0%; overflow: hidden auto"
    >
      <div role="group" aria-labelledby="-trigger" data-slot="select-group" class="scroll-my-1 p-1">
        <div
          role="option"
          aria-labelledby="s1-e0"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
          data-highlighted=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s1-e0">2024</span>
        </div>
        <div
          role="option"
          aria-labelledby="s1-e1"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s1-e1">2025</span>
        </div>
        <div
          role="option"
          aria-labelledby="s1-e2"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s1-e2">2026</span>
        </div>
        <div
          role="option"
          aria-labelledby="s1-e3"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s1-e3">2027</span>
        </div>
        <div
          role="option"
          aria-labelledby="s1-e4"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s1-e4">2028</span>
        </div>
        <div
          role="option"
          aria-labelledby="s1-e5"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s1-e5">2029</span>
        </div>
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/select.js
// shadless select behavior (wireSelect) — registers with the base; multi-instance: every
// [data-slot=select-trigger] "<k>-trigger" ↔ <template id="<k>-tpl">.
(function () {
  shadless.register("select", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=select-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-tpl"));
      if (!tpl) return;
      var valueNode = trigger.querySelector("[data-slot=select-value]");

      // clone once — kernel mounts/unmounts the wrapper around the same content
      var host = tpl.content.cloneNode(true);
      var holder = document.createElement("div");
      holder.appendChild(host);
      var content = holder.querySelector("[data-slot=select-content]");
      var viewport = content.querySelector("[data-slot=select-viewport], [data-radix-select-viewport]") || content.children[1];

      function lock(on) {
        if (on) {
          document.body.setAttribute("data-scroll-locked", "1");
          document.body.style.setProperty("pointer-events", "none");
          content.style.setProperty("pointer-events", "auto");
        } else {
          document.body.removeAttribute("data-scroll-locked");
          document.body.style.removeProperty("pointer-events");
        }
      }

      var handles = RadixKernel.wireSelect({
        trigger: trigger,
        content: content,
        viewport: viewport,
        valueNode: valueNode,
        onClosed: function () { lock(false); shadless.h.emit(trigger, "close", "select"); },
      });

      // the selected option: its value is `value` / `data-value` / id (the
      // React value prop never reaches the DOM, so authors add data-value);
      // with none of those the label text stands in — value and label are
      // then equal, never silently different
      var selected = content.querySelector("[role=option][aria-selected=true]");
      var labelOf = function (item) { return item ? (item.textContent || "").trim() : null; };
      var valueOf = function (item) { return item ? (shadless.h.itemValue(item) || labelOf(item)) : null; };
      var kernelSelect = handles.select;
      handles.select = function (item) {
        var before = valueOf(selected);
        kernelSelect(item);
        selected = item;
        shadless.h.syncForm(trigger);
        var after = valueOf(item);
        if (after !== before) shadless.h.emit(trigger, "change", "select", { value: after, label: labelOf(item), item: item });
      };
      // a trigger carrying `name` submits the selected option's value
      shadless.h.formMirror(trigger, {
        read: function () { return valueOf(selected) },
        write: function (v) {
          var items = content.querySelectorAll("[role=option]");
          for (var i = 0; i < items.length; i++) if (valueOf(items[i]) === v) { kernelSelect(items[i]); selected = items[i]; return; }
        },
      });

      function open() {
        handles.open();
        // kernel hides all body children incl. the trigger (radix keeps it visible)
        trigger.removeAttribute("aria-hidden");
        trigger.removeAttribute("data-aria-hidden");
        lock(true);
        shadless.h.emit(trigger, "open", "select");
      }

      trigger.addEventListener("click", function () {
        handles.isOpen() ? handles.close(true) : open();
      }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "select",
        open: function () { if (!handles.isOpen()) open() },
        close: function () { if (handles.isOpen()) handles.close(true) },
        toggle: function () { handles.isOpen() ? handles.close(true) : open() },
        isOpen: function () { return handles.isOpen() },
        select: function (item) { handles.select(typeof item === "string" ? content.querySelector(item) : item) },
        value: function () { return valueOf(selected) },
        label: function () { return valueNode ? valueNode.textContent : labelOf(selected) },
        selected: function () { return selected },
      })
      // radix opens on Enter/Space/Arrow keys from the trigger
      trigger.addEventListener("keydown", function (e) {
        if (["Enter", " ", "ArrowDown", "ArrowUp"].indexOf(e.key) !== -1 && !handles.isOpen()) {
          e.preventDefault();
          open();
        }
      }, { signal: w.signal });
      // radix closes + selects on item pointerup; kernel only handles keyboard
      content.addEventListener("pointerup", function (e) {
        var item = e.target.closest("[role=option]");
        if (item && viewport.contains(item)) handles.select(item);
      }, { signal: w.signal });
    });
  } })
})()

// js/checkbox.js
// shadless checkbox behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  function set(root, checked, ctx) {
    h.setChecked(root, checked)
    var ind = root.querySelector("[data-slot=checkbox-indicator]")
    if (checked && !ind) {
      var node = h.cloneTemplate(h.findTemplate(ctx, "checkbox-indicator"))
      if (node) root.appendChild(node)
    } else if (!checked && ind) ind.remove()
    h.syncForm(root)
  }
  shadless.register("checkbox", { slots: {
    checkbox: {
      init: function (root) {
        h.formMirror(root, {
          read: function () { return root.getAttribute("aria-checked") === "true" },
          write: function (v) { set(root, v, null) },
        })
      },
      onClick: function (root, ctx) {
        var checked = root.getAttribute("aria-checked") !== "true"
        set(root, checked, ctx)
        h.emit(root, "change", "checkbox", { checked: checked })
      },
    },
    // switch: thumb is always in DOM; root + thumb data-state stay in sync.,
  } })
})()
```
:::

::::


## Installation



**Add shadless and this component to your Tailwind v4 entry:**

```css
@import "shadless";
@import "shadless/field.css";
```

The files this component needs:

| File | Purpose |
| --- | --- |
| `dist/shadless-core.css` | theme + animate layer — self-contained, needs only your tailwindcss build |
| `dist/css/field.css` | this component's slot styles (`@apply` source — your build compiles it) |
| `dist/components/field.html` | component markup — copy your page's structure from here |
| — | no JavaScript: this component is markup + CSS |


**Copy the markup from                              into your page and adapt it — the inline utilities are picked up by your build's content scan.**

No Tailwind build? Use the precompiled `dist/out.css` (every component) as a single stylesheet instead of the imports above.



## Usage

Copy the markup from `dist/components/field.html` and adapt it — every slot
is a `data-slot` attribute, and open/close state is a `data-state` the
runtime drives. The component's API axes are data attributes:

| JSX prop | Markup |
| --- | --- |
| `orientation="outline"` (JSX prop) | `data-orientation="outline"` (markup) |
## Composition

The slot tree — every node is a `data-slot` attribute in the shipped markup:

```text
field
├── field-label
├── Input / Textarea / Switch / Select
├── field-description
└── field-error
```

## Anatomy

The `Field` family is designed for composing accessible forms. A typical field is structured as follows:

```tsx showLineNumbers
<Field>
  <FieldLabel htmlFor="input-id">Label</FieldLabel>
  {/* Input, Select, Switch, etc. */}
  <FieldDescription>Optional helper text.</FieldDescription>
  <FieldError>Validation message.</FieldError>
</Field>
```

- `Field` is the core wrapper for a single field.
- `FieldContent` is a flex column that groups label and description. Not required if you have no description.
- Wrap related fields with `FieldGroup`, and use `FieldSet` with `FieldLegend` for semantic grouping.

## Form

See the Form documentation for building forms with the `Field` component and React Hook Form, Tanstack Form, or Formisch.

## Input

::::demo field-input
<iframe class="demo" src="/demos/field-input.html" title="field-input" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/field-input.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [field-input.html]
<fieldset
  data-slot="field-set"
  class="gap-4 has-[&gt;[data-slot=checkbox-group]]:gap-3 has-[&gt;[data-slot=radio-group]]:gap-3 flex flex-col w-full max-w-xs"
>
  <div
    data-slot="field-group"
    class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col"
  >
    <div
      role="group"
      data-slot="field"
      data-orientation="vertical"
      class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
    >
      <label
        data-slot="field-label"
        class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
        for="username"
        >Username</label
      ><input
        data-slot="input"
        class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
        id="username"
        placeholder="Max Leiter"
        type="text"
      />
      <p
        data-slot="field-description"
        class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        Choose a unique username for your account.
      </p>
    </div>
    <div
      role="group"
      data-slot="field"
      data-orientation="vertical"
      class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
    >
      <label
        data-slot="field-label"
        class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
        for="password"
        >Password</label
      >
      <p
        data-slot="field-description"
        class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        Must be at least 8 characters long.
      </p>
      <input
        data-slot="input"
        class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
        id="password"
        placeholder="••••••••"
        type="password"
      />
    </div>
  </div>
</fieldset>
```
:::

::::


## Textarea

::::demo field-textarea
<iframe class="demo" src="/demos/field-textarea.html" title="field-textarea" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/field-textarea.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [field-textarea.html]
<fieldset
  data-slot="field-set"
  class="gap-4 has-[&gt;[data-slot=checkbox-group]]:gap-3 has-[&gt;[data-slot=radio-group]]:gap-3 flex flex-col w-full max-w-xs"
>
  <div
    data-slot="field-group"
    class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col"
  >
    <div
      role="group"
      data-slot="field"
      data-orientation="vertical"
      class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
    >
      <label
        data-slot="field-label"
        class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
        for="feedback"
        >Feedback</label
      ><textarea
        data-slot="textarea"
        class="border-input dark:bg-input/30 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 rounded-lg border bg-transparent px-2.5 py-2 text-base transition-colors focus-visible:ring-3 aria-invalid:ring-3 md:text-sm flex field-sizing-content min-h-16 w-full outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50"
        id="feedback"
        placeholder="Your feedback helps us improve..."
        rows="4"
      ></textarea>
      <p
        data-slot="field-description"
        class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        Share your thoughts about our service.
      </p>
    </div>
  </div>
</fieldset>
```
:::

::::


## Select

::::demo field-select
<iframe class="demo" src="/demos/field-select.html" title="field-select" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/field-select.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [field-select.html]
<div
  role="group"
  data-slot="field"
  data-orientation="vertical"
  class="data-[invalid=true]:text-destructive gap-2 group/field flex flex-col *:w-full [&amp;&gt;.sr-only]:w-auto w-full max-w-xs"
>
  <label
    data-slot="field-label"
    class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
    >Department</label
  ><button
    type="button"
    role="combobox"
    aria-controls="s0"
    aria-expanded="false"
    aria-autocomplete="none"
    dir="ltr"
    data-state="closed"
    data-placeholder=""
    data-slot="select-trigger"
    data-size="default"
    class="border-input data-placeholder:text-muted-foreground dark:bg-input/30 dark:hover:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 gap-1.5 rounded-lg border bg-transparent py-2 pr-2 pl-2.5 text-sm transition-colors select-none focus-visible:ring-3 aria-invalid:ring-3 data-[size=default]:h-8 data-[size=sm]:h-7 data-[size=sm]:rounded-[min(var(--radius-md),10px)] *:data-[slot=select-value]:gap-1.5 [&amp;_svg:not([class*='size-'])]:size-4 flex w-fit items-center justify-between whitespace-nowrap outline-none disabled:cursor-not-allowed disabled:opacity-50 *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex *:data-[slot=select-value]:items-center [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
    id="s0-trigger"
  >
    <span data-slot="select-value" style="pointer-events: none">Choose department</span
    ><svg
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
      class="lucide lucide-chevron-down text-muted-foreground size-4 pointer-events-none"
    >
      <path d="m6 9 6 6 6-6"></path>
    </svg>
  </button>
  <p
    data-slot="field-description"
    class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
  >
    Select your department or area of work.
  </p>
</div>
<template id="s0-tpl">
  <div
    role="listbox"
    id="s0"
    data-state="open"
    dir="ltr"
    data-slot="select-content"
    data-align-trigger="true"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent relative z-50 max-h-(--radix-select-content-available-height) origin-(--radix-select-content-transform-origin) overflow-x-hidden overflow-y-auto data-[align-trigger=true]:animate-none"
    tabindex="-1"
    style="
      box-sizing: border-box;
      max-height: 100%;
      display: flex;
      flex-direction: column;
      outline: none;
      pointer-events: auto;
    "
  >
    <style>
      [data-radix-select-viewport] {
        scrollbar-width: none;
        -ms-overflow-style: none;
        -webkit-overflow-scrolling: touch;
      }
      [data-radix-select-viewport]::-webkit-scrollbar {
        display: none;
      }
    </style>
    <div
      data-radix-select-viewport=""
      role="presentation"
      data-position="item-aligned"
      class="data-[position=popper]:h-(--radix-select-trigger-height) data-[position=popper]:w-full data-[position=popper]:min-w-(--radix-select-trigger-width)"
      style="position: relative; flex: 1 1 0%; overflow: hidden auto"
    >
      <div role="group" aria-labelledby="-trigger" data-slot="select-group" class="scroll-my-1 p-1">
        <div
          role="option"
          aria-labelledby="s0-e0"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
          data-highlighted=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e0">Engineering</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e1"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e1">Design</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e2"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e2">Marketing</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e3"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e3">Sales</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e4"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e4">Customer Support</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e5"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e5">Human Resources</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e6"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e6">Finance</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e7"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pr-8 pl-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute right-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e7">Operations</span>
        </div>
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/select.js
// shadless select behavior (wireSelect) — registers with the base; multi-instance: every
// [data-slot=select-trigger] "<k>-trigger" ↔ <template id="<k>-tpl">.
(function () {
  shadless.register("select", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=select-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-tpl"));
      if (!tpl) return;
      var valueNode = trigger.querySelector("[data-slot=select-value]");

      // clone once — kernel mounts/unmounts the wrapper around the same content
      var host = tpl.content.cloneNode(true);
      var holder = document.createElement("div");
      holder.appendChild(host);
      var content = holder.querySelector("[data-slot=select-content]");
      var viewport = content.querySelector("[data-slot=select-viewport], [data-radix-select-viewport]") || content.children[1];

      function lock(on) {
        if (on) {
          document.body.setAttribute("data-scroll-locked", "1");
          document.body.style.setProperty("pointer-events", "none");
          content.style.setProperty("pointer-events", "auto");
        } else {
          document.body.removeAttribute("data-scroll-locked");
          document.body.style.removeProperty("pointer-events");
        }
      }

      var handles = RadixKernel.wireSelect({
        trigger: trigger,
        content: content,
        viewport: viewport,
        valueNode: valueNode,
        onClosed: function () { lock(false); shadless.h.emit(trigger, "close", "select"); },
      });

      // the selected option: its value is `value` / `data-value` / id (the
      // React value prop never reaches the DOM, so authors add data-value);
      // with none of those the label text stands in — value and label are
      // then equal, never silently different
      var selected = content.querySelector("[role=option][aria-selected=true]");
      var labelOf = function (item) { return item ? (item.textContent || "").trim() : null; };
      var valueOf = function (item) { return item ? (shadless.h.itemValue(item) || labelOf(item)) : null; };
      var kernelSelect = handles.select;
      handles.select = function (item) {
        var before = valueOf(selected);
        kernelSelect(item);
        selected = item;
        shadless.h.syncForm(trigger);
        var after = valueOf(item);
        if (after !== before) shadless.h.emit(trigger, "change", "select", { value: after, label: labelOf(item), item: item });
      };
      // a trigger carrying `name` submits the selected option's value
      shadless.h.formMirror(trigger, {
        read: function () { return valueOf(selected) },
        write: function (v) {
          var items = content.querySelectorAll("[role=option]");
          for (var i = 0; i < items.length; i++) if (valueOf(items[i]) === v) { kernelSelect(items[i]); selected = items[i]; return; }
        },
      });

      function open() {
        handles.open();
        // kernel hides all body children incl. the trigger (radix keeps it visible)
        trigger.removeAttribute("aria-hidden");
        trigger.removeAttribute("data-aria-hidden");
        lock(true);
        shadless.h.emit(trigger, "open", "select");
      }

      trigger.addEventListener("click", function () {
        handles.isOpen() ? handles.close(true) : open();
      }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "select",
        open: function () { if (!handles.isOpen()) open() },
        close: function () { if (handles.isOpen()) handles.close(true) },
        toggle: function () { handles.isOpen() ? handles.close(true) : open() },
        isOpen: function () { return handles.isOpen() },
        select: function (item) { handles.select(typeof item === "string" ? content.querySelector(item) : item) },
        value: function () { return valueOf(selected) },
        label: function () { return valueNode ? valueNode.textContent : labelOf(selected) },
        selected: function () { return selected },
      })
      // radix opens on Enter/Space/Arrow keys from the trigger
      trigger.addEventListener("keydown", function (e) {
        if (["Enter", " ", "ArrowDown", "ArrowUp"].indexOf(e.key) !== -1 && !handles.isOpen()) {
          e.preventDefault();
          open();
        }
      }, { signal: w.signal });
      // radix closes + selects on item pointerup; kernel only handles keyboard
      content.addEventListener("pointerup", function (e) {
        var item = e.target.closest("[role=option]");
        if (item && viewport.contains(item)) handles.select(item);
      }, { signal: w.signal });
    });
  } })
})()
```
:::

::::


## Slider

::::demo field-slider
<iframe class="demo" src="/demos/field-slider.html" title="field-slider" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/field-slider.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [field-slider.html]
<div
  role="group"
  data-slot="field"
  data-orientation="vertical"
  class="data-[invalid=true]:text-destructive gap-2 group/field flex flex-col *:w-full [&amp;&gt;.sr-only]:w-auto w-full max-w-xs"
>
  <div
    data-slot="field-label"
    class="gap-2 text-sm font-medium group-data-[disabled=true]/field:opacity-50 flex w-fit items-center"
  >
    Price Range
  </div>
  <p
    data-slot="field-description"
    class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
  >
    Set your budget range ($<span class="font-medium tabular-nums">200</span> -
    <span class="font-medium tabular-nums">800</span>).
  </p>
  <span
    dir="ltr"
    data-orientation="horizontal"
    aria-disabled="false"
    data-slot="slider"
    class="data-vertical:min-h-40 relative flex touch-none items-center select-none data-disabled:opacity-50 data-vertical:h-full data-vertical:w-auto data-vertical:flex-col mt-2 w-full"
    aria-label="Price Range"
    style="--radix-slider-thumb-transform: translateX(-50%)"
    ><span
      data-orientation="horizontal"
      data-slot="slider-track"
      class="bg-muted rounded-full data-horizontal:h-1 data-vertical:w-1 relative grow overflow-hidden data-horizontal:w-full data-vertical:h-full"
      ><span
        data-orientation="horizontal"
        data-slot="slider-range"
        class="bg-primary absolute select-none data-horizontal:h-full data-vertical:w-full"
        style="left: 20%; right: 20%"
      ></span></span
    ><span
      style="
        transform: var(--radix-slider-thumb-transform);
        position: absolute;
        left: calc(20% + 0px);
      "
      ><span
        role="slider"
        aria-valuemin="0"
        aria-valuemax="1000"
        aria-orientation="horizontal"
        data-orientation="horizontal"
        tabindex="0"
        data-slot="slider-thumb"
        class="border-ring ring-ring/50 relative size-3 rounded-full border bg-white transition-[color,box-shadow] after:absolute after:-inset-2 hover:ring-3 focus-visible:ring-3 focus-visible:outline-hidden active:ring-3 block shrink-0 select-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
        aria-label="Minimum"
        aria-valuenow="200"
        style=""
      ></span></span
    ><span
      style="
        transform: var(--radix-slider-thumb-transform);
        position: absolute;
        left: calc(80% + 0px);
      "
      ><span
        role="slider"
        aria-valuemin="0"
        aria-valuemax="1000"
        aria-orientation="horizontal"
        data-orientation="horizontal"
        tabindex="0"
        data-slot="slider-thumb"
        class="border-ring ring-ring/50 relative size-3 rounded-full border bg-white transition-[color,box-shadow] after:absolute after:-inset-2 hover:ring-3 focus-visible:ring-3 focus-visible:outline-hidden active:ring-3 block shrink-0 select-none disabled:pointer-events-none disabled:opacity-50"
        data-radix-collection-item=""
        aria-label="Maximum"
        aria-valuenow="800"
        style=""
      ></span></span
  ></span>
</div>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/slider.js
// shadless slider behavior (wireSlider) — registers with the base; multi-instance: every [data-slot=slider]
// root; the initial value(s) come from the thumbs' aria-valuenow.
(function () {
  shadless.register("slider", { init: function (live) {
    var roots = live.querySelectorAll("[data-slot=slider]");
    Array.prototype.forEach.call(roots, function (root) {
      var w = shadless.h.wire(root, live)
      if (!w) return
      w.persistent = true // kernel wireSlider holds root listeners with no unwire
      var thumbs = root.querySelectorAll("[data-slot=slider-thumb]");
      var values = Array.prototype.map.call(thumbs, function (t) { return Number(t.getAttribute("aria-valuenow") || 0); });
      var handles = RadixKernel.wireSlider({
        root: root,
        track: root.querySelector("[data-slot=slider-track]"),
        range: root.querySelector("[data-slot=slider-range]"),
        defaultValue: values.length ? values : [50],
        onValueChange: function (v) { shadless.h.syncForm(root); shadless.h.emit(root, "change", "slider", { values: v.slice() }); },
        // radix onValueCommit: once, when the pointer is released / a key
        // step lands — the value to persist; change is the live stream
        onValueCommit: function (v) { shadless.h.emit(root, "commit", "slider", { values: v.slice() }); },
      });
      // a root carrying `name` submits one input per thumb (radix)
      shadless.h.formMirror(root, {
        read: function () { return handles.values() },
        write: function (v) { v.forEach(function (x, i) { handles.setValue(x, i) }) },
      });
      shadless.instances.set(root, { component: "slider",
        values: function () { return handles.values() },
        setValue: function (value, atIndex, opts) { handles.setValue(value, atIndex || 0, opts) },
      })
    });
  } })
})()
```
:::

::::


## Fieldset

::::demo field-fieldset
<iframe class="demo" src="/demos/field-fieldset.html" title="field-fieldset" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/field-fieldset.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [field-fieldset.html]
<fieldset
  data-slot="field-set"
  class="gap-4 has-[&gt;[data-slot=checkbox-group]]:gap-3 has-[&gt;[data-slot=radio-group]]:gap-3 flex flex-col w-full max-w-sm"
>
  <legend
    data-slot="field-legend"
    data-variant="legend"
    class="mb-1.5 font-medium data-[variant=label]:text-sm data-[variant=legend]:text-base"
  >
    Address Information
  </legend>
  <p
    data-slot="field-description"
    class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
  >
    We need your address to deliver your order.
  </p>
  <div
    data-slot="field-group"
    class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col"
  >
    <div
      role="group"
      data-slot="field"
      data-orientation="vertical"
      class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
    >
      <label
        data-slot="field-label"
        class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
        for="street"
        >Street Address</label
      ><input
        data-slot="input"
        class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
        id="street"
        placeholder="123 Main St"
        type="text"
      />
    </div>
    <div class="grid grid-cols-2 gap-4">
      <div
        role="group"
        data-slot="field"
        data-orientation="vertical"
        class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
      >
        <label
          data-slot="field-label"
          class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
          for="city"
          >City</label
        ><input
          data-slot="input"
          class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
          id="city"
          placeholder="New York"
          type="text"
        />
      </div>
      <div
        role="group"
        data-slot="field"
        data-orientation="vertical"
        class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
      >
        <label
          data-slot="field-label"
          class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
          for="zip"
          >Postal Code</label
        ><input
          data-slot="input"
          class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
          id="zip"
          placeholder="90502"
          type="text"
        />
      </div>
    </div>
  </div>
</fieldset>
```
:::

::::


## Checkbox

::::demo field-checkbox
<iframe class="demo" src="/demos/field-checkbox.html" title="field-checkbox" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/field-checkbox.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [field-checkbox.html]
<div
  data-slot="field-group"
  class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex flex-col w-full max-w-xs"
>
  <fieldset
    data-slot="field-set"
    class="gap-4 has-[&gt;[data-slot=checkbox-group]]:gap-3 has-[&gt;[data-slot=radio-group]]:gap-3 flex flex-col"
  >
    <legend
      data-slot="field-legend"
      data-variant="label"
      class="mb-1.5 font-medium data-[variant=label]:text-sm data-[variant=legend]:text-base"
    >
      Show these items on the desktop
    </legend>
    <p
      data-slot="field-description"
      class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
    >
      Select the items you want to show on the desktop.
    </p>
    <div
      data-slot="field-group"
      class="data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col gap-3"
    >
      <div
        role="group"
        data-slot="field"
        data-orientation="horizontal"
        class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
      >
        <button
          type="button"
          role="checkbox"
          aria-checked="false"
          data-state="unchecked"
          value="on"
          data-slot="checkbox"
          class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
          id="finder-pref-9k2-hard-disks-ljj"
        ></button
        ><label
          data-slot="field-label"
          class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
          for="finder-pref-9k2-hard-disks-ljj"
          >Hard disks</label
        >
      </div>
      <div
        role="group"
        data-slot="field"
        data-orientation="horizontal"
        class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
      >
        <button
          type="button"
          role="checkbox"
          aria-checked="false"
          data-state="unchecked"
          value="on"
          data-slot="checkbox"
          class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
          id="finder-pref-9k2-external-disks-1yg"
        ></button
        ><label
          data-slot="field-label"
          class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
          for="finder-pref-9k2-external-disks-1yg"
          >External disks</label
        >
      </div>
      <div
        role="group"
        data-slot="field"
        data-orientation="horizontal"
        class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
      >
        <button
          type="button"
          role="checkbox"
          aria-checked="false"
          data-state="unchecked"
          value="on"
          data-slot="checkbox"
          class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
          id="finder-pref-9k2-cds-dvds-fzt"
        ></button
        ><label
          data-slot="field-label"
          class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
          for="finder-pref-9k2-cds-dvds-fzt"
          >CDs, DVDs, and iPods</label
        >
      </div>
      <div
        role="group"
        data-slot="field"
        data-orientation="horizontal"
        class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
      >
        <button
          type="button"
          role="checkbox"
          aria-checked="false"
          data-state="unchecked"
          value="on"
          data-slot="checkbox"
          class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
          id="finder-pref-9k2-connected-servers-6l2"
        ></button
        ><label
          data-slot="field-label"
          class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
          for="finder-pref-9k2-connected-servers-6l2"
          >Connected servers</label
        >
      </div>
    </div>
  </fieldset>
  <div
    data-slot="field-separator"
    data-content="false"
    class="-my-2 h-5 text-sm group-data-[variant=outline]/field-group:-mb-2 relative"
  >
    <div
      data-orientation="horizontal"
      role="none"
      data-slot="separator"
      class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch absolute inset-0 top-1/2"
    ></div>
  </div>
  <div
    role="group"
    data-slot="field"
    data-orientation="horizontal"
    class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
  >
    <button
      type="button"
      role="checkbox"
      aria-checked="true"
      data-state="checked"
      value="on"
      data-slot="checkbox"
      class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
      id="finder-pref-9k2-sync-folders-nep"
    >
      <span
        data-state="checked"
        data-slot="checkbox-indicator"
        class="[&amp;&gt;svg]:size-3.5 grid place-content-center text-current transition-none"
        style="pointer-events: none"
        ><svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="lucide lucide-check"
        >
          <path d="M20 6 9 17l-5-5"></path></svg
      ></span>
    </button>
    <div
      data-slot="field-content"
      class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"
    >
      <label
        data-slot="field-label"
        class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
        for="finder-pref-9k2-sync-folders-nep"
        >Sync Desktop &amp; Documents folders</label
      >
      <p
        data-slot="field-description"
        class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        Your Desktop &amp; Documents folders are being synced with iCloud Drive. You can access them
        from other devices.
      </p>
    </div>
  </div>
</div>
```
:::

::::


## Radio

::::demo field-radio
<iframe class="demo" src="/demos/field-radio.html" title="field-radio" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/field-radio.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [field-radio.html]
<fieldset
  data-slot="field-set"
  class="gap-4 has-[&gt;[data-slot=checkbox-group]]:gap-3 has-[&gt;[data-slot=radio-group]]:gap-3 flex flex-col w-full max-w-xs"
>
  <legend
    data-slot="field-legend"
    data-variant="label"
    class="mb-1.5 font-medium data-[variant=label]:text-sm data-[variant=legend]:text-base"
  >
    Subscription Plan
  </legend>
  <p
    data-slot="field-description"
    class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
  >
    Yearly and lifetime plans offer significant savings.
  </p>
  <div
    role="radiogroup"
    aria-required="false"
    dir="ltr"
    data-slot="radio-group"
    class="grid gap-2 w-full"
    tabindex="0"
    style="outline: none"
  >
    <div
      role="group"
      data-slot="field"
      data-orientation="horizontal"
      class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
    >
      <button
        type="button"
        role="radio"
        aria-checked="true"
        data-state="checked"
        value="monthly"
        data-slot="radio-group-item"
        class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
        id="plan-monthly"
        tabindex="-1"
        data-radix-collection-item=""
      >
        <span
          data-state="checked"
          data-slot="radio-group-indicator"
          class="flex size-4 items-center justify-center"
          ><span
            class="bg-primary-foreground absolute top-1/2 left-1/2 size-2 -translate-x-1/2 -translate-y-1/2 rounded-full"
          ></span
        ></span></button
      ><label
        data-slot="field-label"
        class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
        for="plan-monthly"
        >Monthly ($9.99/month)</label
      >
    </div>
    <div
      role="group"
      data-slot="field"
      data-orientation="horizontal"
      class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
    >
      <button
        type="button"
        role="radio"
        aria-checked="false"
        data-state="unchecked"
        value="yearly"
        data-slot="radio-group-item"
        class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
        id="plan-yearly"
        tabindex="-1"
        data-radix-collection-item=""
      ></button
      ><label
        data-slot="field-label"
        class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
        for="plan-yearly"
        >Yearly ($99.99/year)</label
      >
    </div>
    <div
      role="group"
      data-slot="field"
      data-orientation="horizontal"
      class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
    >
      <button
        type="button"
        role="radio"
        aria-checked="false"
        data-state="unchecked"
        value="lifetime"
        data-slot="radio-group-item"
        class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
        id="plan-lifetime"
        tabindex="-1"
        data-radix-collection-item=""
      ></button
      ><label
        data-slot="field-label"
        class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
        for="plan-lifetime"
        >Lifetime ($299.99)</label
      >
    </div>
  </div>
</fieldset>
```
:::

::::


## Switch

::::demo field-switch
<iframe class="demo" src="/demos/field-switch.html" title="field-switch" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/field-switch.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [field-switch.html]
<div
  role="group"
  data-slot="field"
  data-orientation="horizontal"
  class="data-[invalid=true]:text-destructive gap-2 group/field flex flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px w-fit"
>
  <label
    data-slot="field-label"
    class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
    for="2fa"
    >Multi-factor authentication</label
  ><button
    type="button"
    role="switch"
    aria-checked="false"
    data-state="unchecked"
    value="on"
    data-slot="switch"
    data-size="default"
    class="data-checked:bg-primary data-unchecked:bg-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 dark:data-unchecked:bg-input/80 shrink-0 rounded-full border border-transparent focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:border-transparent data-[size=default]:h-[18.4px] data-[size=default]:w-[32px] data-[size=sm]:h-[14px] data-[size=sm]:w-[24px] peer group/switch relative inline-flex items-center transition-all outline-none after:absolute after:-inset-x-3 after:-inset-y-2 data-disabled:cursor-not-allowed data-disabled:opacity-50"
    id="2fa"
  >
    <span
      data-state="unchecked"
      data-slot="switch-thumb"
      class="bg-background dark:data-unchecked:bg-foreground dark:data-checked:bg-primary-foreground rounded-full group-data-[size=default]/switch:size-4 group-data-[size=sm]/switch:size-3 group-data-[size=default]/switch:data-checked:translate-x-[calc(100%-2px)] group-data-[size=sm]/switch:data-checked:translate-x-[calc(100%-2px)] group-data-[size=default]/switch:data-unchecked:translate-x-0 group-data-[size=sm]/switch:data-unchecked:translate-x-0 pointer-events-none block ring-0 transition-transform"
    ></span>
  </button>
</div>
```
:::

::::


## Choice Card

Wrap `Field` components inside `FieldLabel` to create selectable field groups. This works with `RadioItem`, `Checkbox` and `Switch` components.

::::demo field-choice-card
<iframe class="demo" src="/demos/field-choice-card.html" title="field-choice-card" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/field-choice-card.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [field-choice-card.html]
<div
  data-slot="field-group"
  class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex flex-col w-full max-w-xs"
>
  <fieldset
    data-slot="field-set"
    class="gap-4 has-[&gt;[data-slot=checkbox-group]]:gap-3 has-[&gt;[data-slot=radio-group]]:gap-3 flex flex-col"
  >
    <legend
      data-slot="field-legend"
      data-variant="label"
      class="mb-1.5 font-medium data-[variant=label]:text-sm data-[variant=legend]:text-base"
    >
      Compute Environment
    </legend>
    <p
      data-slot="field-description"
      class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
    >
      Select the compute environment for your cluster.
    </p>
    <div
      role="radiogroup"
      aria-required="false"
      dir="ltr"
      data-slot="radio-group"
      class="grid gap-2 w-full"
      tabindex="0"
      style="outline: none"
    >
      <label
        data-slot="field-label"
        class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
        for="kubernetes-r2h"
        ><div
          role="group"
          data-slot="field"
          data-orientation="horizontal"
          class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
        >
          <div
            data-slot="field-content"
            class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"
          >
            <div
              data-slot="field-label"
              class="gap-2 text-sm font-medium group-data-[disabled=true]/field:opacity-50 flex w-fit items-center"
            >
              Kubernetes
            </div>
            <p
              data-slot="field-description"
              class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
            >
              Run GPU workloads on a K8s cluster.
            </p>
          </div>
          <button
            type="button"
            role="radio"
            aria-checked="true"
            data-state="checked"
            value="kubernetes"
            data-slot="radio-group-item"
            class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
            id="kubernetes-r2h"
            tabindex="-1"
            data-radix-collection-item=""
          >
            <span
              data-state="checked"
              data-slot="radio-group-indicator"
              class="flex size-4 items-center justify-center"
              ><span
                class="bg-primary-foreground absolute top-1/2 left-1/2 size-2 -translate-x-1/2 -translate-y-1/2 rounded-full"
              ></span
            ></span>
          </button></div></label
      ><label
        data-slot="field-label"
        class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
        for="vm-z4k"
        ><div
          role="group"
          data-slot="field"
          data-orientation="horizontal"
          class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
        >
          <div
            data-slot="field-content"
            class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"
          >
            <div
              data-slot="field-label"
              class="gap-2 text-sm font-medium group-data-[disabled=true]/field:opacity-50 flex w-fit items-center"
            >
              Virtual Machine
            </div>
            <p
              data-slot="field-description"
              class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
            >
              Access a cluster to run GPU workloads.
            </p>
          </div>
          <button
            type="button"
            role="radio"
            aria-checked="false"
            data-state="unchecked"
            value="vm"
            data-slot="radio-group-item"
            class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:aria-invalid:border-destructive/50 flex size-4 rounded-full focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary group/radio-group-item peer relative aspect-square shrink-0 border outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
            id="vm-z4k"
            tabindex="-1"
            data-radix-collection-item=""
          ></button></div
      ></label>
    </div>
  </fieldset>
</div>
```
:::

::::


## Field Group

Stack `Field` components with `FieldGroup`. Add `FieldSeparator` to divide them.

::::demo field-group
<iframe class="demo" src="/demos/field-group.html" title="field-group" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/field-group.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [field-group.html]
<div
  data-slot="field-group"
  class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex flex-col w-full max-w-xs"
>
  <fieldset
    data-slot="field-set"
    class="gap-4 has-[&gt;[data-slot=checkbox-group]]:gap-3 has-[&gt;[data-slot=radio-group]]:gap-3 flex flex-col"
  >
    <label
      data-slot="field-label"
      class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
      >Responses</label
    >
    <p
      data-slot="field-description"
      class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
    >
      Get notified when ChatGPT responds to requests that take time, like research or image
      generation.
    </p>
    <div
      data-slot="checkbox-group"
      class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col"
    >
      <div
        role="group"
        data-slot="field"
        data-orientation="horizontal"
        class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
      >
        <button
          type="button"
          role="checkbox"
          aria-checked="true"
          data-state="checked"
          data-disabled=""
          disabled=""
          value="on"
          data-slot="checkbox"
          class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
          id="push"
        >
          <span
            data-state="checked"
            data-disabled=""
            data-slot="checkbox-indicator"
            class="[&amp;&gt;svg]:size-3.5 grid place-content-center text-current transition-none"
            style="pointer-events: none"
            ><svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              class="lucide lucide-check"
            >
              <path d="M20 6 9 17l-5-5"></path></svg
          ></span></button
        ><label
          data-slot="field-label"
          class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
          for="push"
          >Push notifications</label
        >
      </div>
    </div>
  </fieldset>
  <div
    data-slot="field-separator"
    data-content="false"
    class="-my-2 h-5 text-sm group-data-[variant=outline]/field-group:-mb-2 relative"
  >
    <div
      data-orientation="horizontal"
      role="none"
      data-slot="separator"
      class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch absolute inset-0 top-1/2"
    ></div>
  </div>
  <fieldset
    data-slot="field-set"
    class="gap-4 has-[&gt;[data-slot=checkbox-group]]:gap-3 has-[&gt;[data-slot=radio-group]]:gap-3 flex flex-col"
  >
    <label
      data-slot="field-label"
      class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
      >Tasks</label
    >
    <p
      data-slot="field-description"
      class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
    >
      Get notified when tasks you've created have updates. <a href="#">Manage tasks</a>
    </p>
    <div
      data-slot="checkbox-group"
      class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col"
    >
      <div
        role="group"
        data-slot="field"
        data-orientation="horizontal"
        class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
      >
        <button
          type="button"
          role="checkbox"
          aria-checked="false"
          data-state="unchecked"
          value="on"
          data-slot="checkbox"
          class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
          id="push-tasks"
        ></button
        ><label
          data-slot="field-label"
          class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
          for="push-tasks"
          >Push notifications</label
        >
      </div>
      <div
        role="group"
        data-slot="field"
        data-orientation="horizontal"
        class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
      >
        <button
          type="button"
          role="checkbox"
          aria-checked="false"
          data-state="unchecked"
          value="on"
          data-slot="checkbox"
          class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
          id="email-tasks"
        ></button
        ><label
          data-slot="field-label"
          class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
          for="email-tasks"
          >Email notifications</label
        >
      </div>
    </div>
  </fieldset>
</div>
```
:::

::::


## RTL

To enable RTL support in shadcn/ui, see the [RTL configuration guide](/guides/rtl).

::::demo field-rtl
<iframe class="demo" src="/demos/field-rtl.html" title="field-rtl" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/field-rtl.html">Open the demo page</a> · <a href="/demos/field-rtl-he.html">HE</a> · <a href="/demos/field-rtl-en.html">EN</a></p>

::: code-group
```text:line-numbers [field-rtl.html]
<div class="w-full max-w-md py-6" dir="rtl">
  <form>
    <div
      data-slot="field-group"
      class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col"
    >
      <fieldset
        data-slot="field-set"
        class="gap-4 has-[&gt;[data-slot=checkbox-group]]:gap-3 has-[&gt;[data-slot=radio-group]]:gap-3 flex flex-col"
      >
        <legend
          data-slot="field-legend"
          data-variant="legend"
          class="mb-1.5 font-medium data-[variant=label]:text-sm data-[variant=legend]:text-base"
        >
          طريقة الدفع
        </legend>
        <p
          data-slot="field-description"
          class="text-muted-foreground text-start text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
        >
          جميع المعاملات آمنة ومشفرة
        </p>
        <div
          data-slot="field-group"
          class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col"
        >
          <div
            role="group"
            data-slot="field"
            data-orientation="vertical"
            class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
          >
            <label
              data-slot="field-label"
              class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
              for="checkout-7j9-card-name-43j-rtl"
              >الاسم على البطاقة</label
            ><input
              data-slot="input"
              class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
              id="checkout-7j9-card-name-43j-rtl"
              placeholder="Evil Rabbit"
              required=""
            />
          </div>
          <div
            role="group"
            data-slot="field"
            data-orientation="vertical"
            class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
          >
            <label
              data-slot="field-label"
              class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
              for="checkout-7j9-card-number-uw1-rtl"
              >رقم البطاقة</label
            ><input
              data-slot="input"
              class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
              id="checkout-7j9-card-number-uw1-rtl"
              placeholder="1234 5678 9012 3456"
              required=""
            />
            <p
              data-slot="field-description"
              class="text-muted-foreground text-start text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
            >
              أدخل رقم البطاقة المكون من 16 رقمًا
            </p>
          </div>
          <div class="grid grid-cols-3 gap-4">
            <div
              role="group"
              data-slot="field"
              data-orientation="vertical"
              class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
            >
              <label
                data-slot="field-label"
                class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
                for="checkout-exp-month-ts6-rtl"
                >الشهر</label
              ><button
                type="button"
                role="combobox"
                aria-controls="s0"
                aria-expanded="false"
                aria-autocomplete="none"
                dir="rtl"
                data-state="closed"
                data-placeholder=""
                data-slot="select-trigger"
                data-size="default"
                class="border-input data-placeholder:text-muted-foreground dark:bg-input/30 dark:hover:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 gap-1.5 rounded-lg border bg-transparent py-2 pe-2 ps-2.5 text-sm transition-colors select-none focus-visible:ring-3 aria-invalid:ring-3 data-[size=default]:h-8 data-[size=sm]:h-7 data-[size=sm]:rounded-[min(var(--radius-md),10px)] *:data-[slot=select-value]:gap-1.5 [&amp;_svg:not([class*='size-'])]:size-4 flex w-fit items-center justify-between whitespace-nowrap outline-none disabled:cursor-not-allowed disabled:opacity-50 *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex *:data-[slot=select-value]:items-center [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
                id="s0-trigger"
              >
                <span data-slot="select-value" style="pointer-events: none"></span
                ><svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="24"
                  height="24"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  class="lucide lucide-chevron-down text-muted-foreground size-4 pointer-events-none"
                >
                  <path d="m6 9 6 6 6-6"></path>
                </svg></button
              ><select
                tabindex="-1"
                style="
                  position: absolute;
                  border: 0px;
                  width: 1px;
                  height: 1px;
                  padding: 0px;
                  margin: -1px;
                  overflow: hidden;
                  clip: rect(0px, 0px, 0px, 0px);
                  white-space: nowrap;
                  overflow-wrap: normal;
                "
              >
                <option value=""></option>
                <option value="MM">ش.ش</option>
                <option value="01">٠١</option>
                <option value="02">٠٢</option>
                <option value="03">٠٣</option>
                <option value="04">٠٤</option>
                <option value="05">٠٥</option>
                <option value="06">٠٦</option>
                <option value="07">٠٧</option>
                <option value="08">٠٨</option>
                <option value="09">٠٩</option>
                <option value="10">١٠</option>
                <option value="11">١١</option>
                <option value="12">١٢</option>
              </select>
            </div>
            <div
              role="group"
              data-slot="field"
              data-orientation="vertical"
              class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
            >
              <label
                data-slot="field-label"
                class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
                for="checkout-7j9-exp-year-f59-rtl"
                >السنة</label
              ><button
                type="button"
                role="combobox"
                aria-controls="s1"
                aria-expanded="false"
                aria-autocomplete="none"
                dir="rtl"
                data-state="closed"
                data-placeholder=""
                data-slot="select-trigger"
                data-size="default"
                class="border-input data-placeholder:text-muted-foreground dark:bg-input/30 dark:hover:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 gap-1.5 rounded-lg border bg-transparent py-2 pe-2 ps-2.5 text-sm transition-colors select-none focus-visible:ring-3 aria-invalid:ring-3 data-[size=default]:h-8 data-[size=sm]:h-7 data-[size=sm]:rounded-[min(var(--radius-md),10px)] *:data-[slot=select-value]:gap-1.5 [&amp;_svg:not([class*='size-'])]:size-4 flex w-fit items-center justify-between whitespace-nowrap outline-none disabled:cursor-not-allowed disabled:opacity-50 *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex *:data-[slot=select-value]:items-center [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
                id="s1-trigger"
              >
                <span data-slot="select-value" style="pointer-events: none"></span
                ><svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="24"
                  height="24"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  class="lucide lucide-chevron-down text-muted-foreground size-4 pointer-events-none"
                >
                  <path d="m6 9 6 6 6-6"></path>
                </svg></button
              ><select
                tabindex="-1"
                style="
                  position: absolute;
                  border: 0px;
                  width: 1px;
                  height: 1px;
                  padding: 0px;
                  margin: -1px;
                  overflow: hidden;
                  clip: rect(0px, 0px, 0px, 0px);
                  white-space: nowrap;
                  overflow-wrap: normal;
                "
              >
                <option value=""></option>
                <option value="YYYY">YYYY</option>
                <option value="2024">2024</option>
                <option value="2025">2025</option>
                <option value="2026">2026</option>
                <option value="2027">2027</option>
                <option value="2028">2028</option>
                <option value="2029">2029</option>
              </select>
            </div>
            <div
              role="group"
              data-slot="field"
              data-orientation="vertical"
              class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
            >
              <label
                data-slot="field-label"
                class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
                for="checkout-7j9-cvv-rtl"
                >CVV</label
              ><input
                data-slot="input"
                class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
                id="checkout-7j9-cvv-rtl"
                placeholder="123"
                required=""
              />
            </div>
          </div>
        </div>
      </fieldset>
      <div
        data-slot="field-separator"
        data-content="false"
        class="-my-2 h-5 text-sm group-data-[variant=outline]/field-group:-mb-2 relative"
      >
        <div
          data-orientation="horizontal"
          role="none"
          data-slot="separator"
          class="shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch absolute inset-0 top-1/2"
        ></div>
      </div>
      <fieldset
        data-slot="field-set"
        class="gap-4 has-[&gt;[data-slot=checkbox-group]]:gap-3 has-[&gt;[data-slot=radio-group]]:gap-3 flex flex-col"
      >
        <legend
          data-slot="field-legend"
          data-variant="legend"
          class="mb-1.5 font-medium data-[variant=label]:text-sm data-[variant=legend]:text-base"
        >
          عنوان الفوترة
        </legend>
        <p
          data-slot="field-description"
          class="text-muted-foreground text-start text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
        >
          عنوان الفوترة المرتبط بطريقة الدفع الخاصة بك
        </p>
        <div
          data-slot="field-group"
          class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col"
        >
          <div
            role="group"
            data-slot="field"
            data-orientation="horizontal"
            class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
          >
            <button
              type="button"
              role="checkbox"
              aria-checked="true"
              data-state="checked"
              value="on"
              data-slot="checkbox"
              class="border-input dark:bg-input/30 data-checked:bg-primary data-checked:text-primary-foreground dark:data-checked:bg-primary data-checked:border-primary aria-invalid:aria-checked:border-primary aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 flex size-4 items-center justify-center rounded-[4px] border transition-colors group-has-disabled/field:opacity-50 focus-visible:ring-3 aria-invalid:ring-3 group-has-[:focus-visible]/field-label:ring-0 group-has-[:focus-visible]/field-label:not-data-checked:border-input group-has-[:focus-visible]/field-label:data-checked:border-primary peer relative shrink-0 outline-none after:absolute after:-inset-x-3 after:-inset-y-2 disabled:cursor-not-allowed disabled:opacity-50"
              id="checkout-7j9-same-as-shipping-wgm-rtl"
            >
              <span
                data-state="checked"
                data-slot="checkbox-indicator"
                class="[&amp;&gt;svg]:size-3.5 grid place-content-center text-current transition-none"
                style="pointer-events: none"
                ><svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="24"
                  height="24"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  class="lucide lucide-check"
                >
                  <path d="M20 6 9 17l-5-5"></path></svg
              ></span></button
            ><input
              tabindex="-1"
              type="checkbox"
              value="on"
              checked=""
              style="
                position: absolute;
                pointer-events: none;
                opacity: 0;
                margin: 0px;
                transform: translateX(-100%);
                width: 40px;
                height: 33px;
              "
            /><label
              data-slot="field-label"
              class="text-sm group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col font-normal"
              for="checkout-7j9-same-as-shipping-wgm-rtl"
              >نفس عنوان الشحن</label
            >
          </div>
        </div>
      </fieldset>
      <fieldset
        data-slot="field-set"
        class="gap-4 has-[&gt;[data-slot=checkbox-group]]:gap-3 has-[&gt;[data-slot=radio-group]]:gap-3 flex flex-col"
      >
        <div
          data-slot="field-group"
          class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col"
        >
          <div
            role="group"
            data-slot="field"
            data-orientation="vertical"
            class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full [&amp;&gt;.sr-only]:w-auto"
          >
            <label
              data-slot="field-label"
              class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
              for="checkout-7j9-optional-comments-rtl"
              >تعليقات</label
            ><textarea
              data-slot="textarea"
              class="border-input dark:bg-input/30 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 rounded-lg border bg-transparent px-2.5 py-2 text-base transition-colors focus-visible:ring-3 aria-invalid:ring-3 md:text-sm flex field-sizing-content min-h-16 w-full outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50 resize-none"
              id="checkout-7j9-optional-comments-rtl"
              placeholder="أضف أي تعليقات إضافية"
            ></textarea>
          </div>
        </div>
      </fieldset>
      <div
        role="group"
        data-slot="field"
        data-orientation="horizontal"
        class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-row items-center has-[&gt;[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
      >
        <button
          data-slot="button"
          data-variant="default"
          data-size="default"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
          type="submit"
        >
          إرسال</button
        ><button
          data-slot="button"
          data-variant="outline"
          data-size="default"
          class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pe-2 has-data-[icon=inline-start]:ps-2"
          type="button"
        >
          إلغاء
        </button>
      </div>
    </div>
  </form>
</div>
<template id="s0-tpl">
  <div
    role="listbox"
    id="s0"
    data-state="open"
    dir="rtl"
    data-slot="select-content"
    data-align-trigger="true"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent relative z-50 max-h-(--radix-select-content-available-height) origin-(--radix-select-content-transform-origin) overflow-x-hidden overflow-y-auto data-[align-trigger=true]:animate-none"
    tabindex="-1"
    style="
      box-sizing: border-box;
      max-height: 100%;
      display: flex;
      flex-direction: column;
      outline: none;
      pointer-events: auto;
    "
  >
    <style>
      [data-radix-select-viewport] {
        scrollbar-width: none;
        -ms-overflow-style: none;
        -webkit-overflow-scrolling: touch;
      }
      [data-radix-select-viewport]::-webkit-scrollbar {
        display: none;
      }
    </style>
    <div
      data-radix-select-viewport=""
      role="presentation"
      data-position="item-aligned"
      class="data-[position=popper]:h-(--radix-select-trigger-height) data-[position=popper]:w-full data-[position=popper]:min-w-(--radix-select-trigger-width)"
      style="position: relative; flex: 1 1 0%; overflow: hidden auto"
    >
      <div role="group" aria-labelledby="-trigger" data-slot="select-group" class="scroll-my-1 p-1">
        <div
          role="option"
          aria-labelledby="s0-e0"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
          data-highlighted=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e0">ش.ش</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e1"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e1">٠١</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e2"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e2">٠٢</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e3"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e3">٠٣</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e4"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e4">٠٤</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e5"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e5">٠٥</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e6"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e6">٠٦</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e7"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e7">٠٧</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e8"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e8">٠٨</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e9"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e9">٠٩</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e10"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e10">١٠</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e11"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e11">١١</span>
        </div>
        <div
          role="option"
          aria-labelledby="s0-e12"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s0-e12">١٢</span>
        </div>
      </div>
    </div>
  </div>
</template>
<template id="s1-tpl">
  <div
    role="listbox"
    id="s1"
    data-state="open"
    dir="rtl"
    data-slot="select-content"
    data-align-trigger="true"
    class="bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-lg shadow-md ring-1 duration-100 cn-menu-target cn-menu-translucent relative z-50 max-h-(--radix-select-content-available-height) origin-(--radix-select-content-transform-origin) overflow-x-hidden overflow-y-auto data-[align-trigger=true]:animate-none"
    tabindex="-1"
    style="
      box-sizing: border-box;
      max-height: 100%;
      display: flex;
      flex-direction: column;
      outline: none;
      pointer-events: auto;
    "
  >
    <style>
      [data-radix-select-viewport] {
        scrollbar-width: none;
        -ms-overflow-style: none;
        -webkit-overflow-scrolling: touch;
      }
      [data-radix-select-viewport]::-webkit-scrollbar {
        display: none;
      }
    </style>
    <div
      data-radix-select-viewport=""
      role="presentation"
      data-position="item-aligned"
      class="data-[position=popper]:h-(--radix-select-trigger-height) data-[position=popper]:w-full data-[position=popper]:min-w-(--radix-select-trigger-width)"
      style="position: relative; flex: 1 1 0%; overflow: hidden auto"
    >
      <div role="group" aria-labelledby="-trigger" data-slot="select-group" class="scroll-my-1 p-1">
        <div
          role="option"
          aria-labelledby="s1-e0"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
          data-highlighted=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s1-e0">YYYY</span>
        </div>
        <div
          role="option"
          aria-labelledby="s1-e1"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s1-e1">2024</span>
        </div>
        <div
          role="option"
          aria-labelledby="s1-e2"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s1-e2">2025</span>
        </div>
        <div
          role="option"
          aria-labelledby="s1-e3"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s1-e3">2026</span>
        </div>
        <div
          role="option"
          aria-labelledby="s1-e4"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s1-e4">2027</span>
        </div>
        <div
          role="option"
          aria-labelledby="s1-e5"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s1-e5">2028</span>
        </div>
        <div
          role="option"
          aria-labelledby="s1-e6"
          aria-selected="false"
          data-state="unchecked"
          tabindex="-1"
          data-slot="select-item"
          class="focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-1.5 rounded-md py-1 pe-8 ps-1.5 text-sm [&amp;_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0"
          data-radix-collection-item=""
        >
          <span
            class="pointer-events-none absolute end-2 flex size-4 items-center justify-center"
          ></span
          ><span id="s1-e6">2029</span>
        </div>
      </div>
    </div>
  </div>
</template>
```

```js:line-numbers [behavior]
// <script src="shadless.js"></script>  — the shared runtime (see Installation)

// js/select.js
// shadless select behavior (wireSelect) — registers with the base; multi-instance: every
// [data-slot=select-trigger] "<k>-trigger" ↔ <template id="<k>-tpl">.
(function () {
  shadless.register("select", { init: function (live) {
    var triggers = live.querySelectorAll("[data-slot=select-trigger][id$='-trigger']");
    Array.prototype.forEach.call(triggers, function (trigger) {
      var w = shadless.h.wire(trigger, live)
      if (!w) return
      var tpl = document.getElementById(trigger.id.replace(/-trigger$/, "-tpl"));
      if (!tpl) return;
      var valueNode = trigger.querySelector("[data-slot=select-value]");

      // clone once — kernel mounts/unmounts the wrapper around the same content
      var host = tpl.content.cloneNode(true);
      var holder = document.createElement("div");
      holder.appendChild(host);
      var content = holder.querySelector("[data-slot=select-content]");
      var viewport = content.querySelector("[data-slot=select-viewport], [data-radix-select-viewport]") || content.children[1];

      function lock(on) {
        if (on) {
          document.body.setAttribute("data-scroll-locked", "1");
          document.body.style.setProperty("pointer-events", "none");
          content.style.setProperty("pointer-events", "auto");
        } else {
          document.body.removeAttribute("data-scroll-locked");
          document.body.style.removeProperty("pointer-events");
        }
      }

      var handles = RadixKernel.wireSelect({
        trigger: trigger,
        content: content,
        viewport: viewport,
        valueNode: valueNode,
        onClosed: function () { lock(false); shadless.h.emit(trigger, "close", "select"); },
      });

      // the selected option: its value is `value` / `data-value` / id (the
      // React value prop never reaches the DOM, so authors add data-value);
      // with none of those the label text stands in — value and label are
      // then equal, never silently different
      var selected = content.querySelector("[role=option][aria-selected=true]");
      var labelOf = function (item) { return item ? (item.textContent || "").trim() : null; };
      var valueOf = function (item) { return item ? (shadless.h.itemValue(item) || labelOf(item)) : null; };
      var kernelSelect = handles.select;
      handles.select = function (item) {
        var before = valueOf(selected);
        kernelSelect(item);
        selected = item;
        shadless.h.syncForm(trigger);
        var after = valueOf(item);
        if (after !== before) shadless.h.emit(trigger, "change", "select", { value: after, label: labelOf(item), item: item });
      };
      // a trigger carrying `name` submits the selected option's value
      shadless.h.formMirror(trigger, {
        read: function () { return valueOf(selected) },
        write: function (v) {
          var items = content.querySelectorAll("[role=option]");
          for (var i = 0; i < items.length; i++) if (valueOf(items[i]) === v) { kernelSelect(items[i]); selected = items[i]; return; }
        },
      });

      function open() {
        handles.open();
        // kernel hides all body children incl. the trigger (radix keeps it visible)
        trigger.removeAttribute("aria-hidden");
        trigger.removeAttribute("data-aria-hidden");
        lock(true);
        shadless.h.emit(trigger, "open", "select");
      }

      trigger.addEventListener("click", function () {
        handles.isOpen() ? handles.close(true) : open();
      }, { signal: w.signal });
      shadless.instances.set(trigger, { component: "select",
        open: function () { if (!handles.isOpen()) open() },
        close: function () { if (handles.isOpen()) handles.close(true) },
        toggle: function () { handles.isOpen() ? handles.close(true) : open() },
        isOpen: function () { return handles.isOpen() },
        select: function (item) { handles.select(typeof item === "string" ? content.querySelector(item) : item) },
        value: function () { return valueOf(selected) },
        label: function () { return valueNode ? valueNode.textContent : labelOf(selected) },
        selected: function () { return selected },
      })
      // radix opens on Enter/Space/Arrow keys from the trigger
      trigger.addEventListener("keydown", function (e) {
        if (["Enter", " ", "ArrowDown", "ArrowUp"].indexOf(e.key) !== -1 && !handles.isOpen()) {
          e.preventDefault();
          open();
        }
      }, { signal: w.signal });
      // radix closes + selects on item pointerup; kernel only handles keyboard
      content.addEventListener("pointerup", function (e) {
        var item = e.target.closest("[role=option]");
        if (item && viewport.contains(item)) handles.select(item);
      }, { signal: w.signal });
    });
  } })
})()

// js/checkbox.js
// shadless checkbox behavior — registers with the base (dist/shadless.js).
// Radix semantics as measured from the shadcn oracle; zero classes added.
(function () {
  var h = shadless.h
  function set(root, checked, ctx) {
    h.setChecked(root, checked)
    var ind = root.querySelector("[data-slot=checkbox-indicator]")
    if (checked && !ind) {
      var node = h.cloneTemplate(h.findTemplate(ctx, "checkbox-indicator"))
      if (node) root.appendChild(node)
    } else if (!checked && ind) ind.remove()
    h.syncForm(root)
  }
  shadless.register("checkbox", { slots: {
    checkbox: {
      init: function (root) {
        h.formMirror(root, {
          read: function () { return root.getAttribute("aria-checked") === "true" },
          write: function (v) { set(root, v, null) },
        })
      },
      onClick: function (root, ctx) {
        var checked = root.getAttribute("aria-checked") !== "true"
        set(root, checked, ctx)
        h.emit(root, "change", "checkbox", { checked: checked })
      },
    },
    // switch: thumb is always in DOM; root + thumb data-state stay in sync.,
  } })
})()
```
:::

::::


## Responsive Layout

- **Vertical fields:** Default orientation stacks label, control, and helper text—ideal for mobile-first layouts.
- **Horizontal fields:** Set `orientation="horizontal"` on `Field` to align the label and control side-by-side. Pair with `FieldContent` to keep descriptions aligned.
- **Responsive fields:** Set `orientation="responsive"` for automatic column layouts inside container-aware parents. Apply `@container/field-group` classes on `FieldGroup` to switch orientations at specific breakpoints.

::::demo field-responsive
<iframe class="demo" src="/demos/field-responsive.html" title="field-responsive" data-status="authored" loading="lazy"></iframe>

<p class="demo-langs"><a href="/demos/field-responsive.html">Open the demo page</a></p>

::: code-group
```text:line-numbers [field-responsive.html]
<div class="w-full max-w-lg">
  <form>
    <fieldset
      data-slot="field-set"
      class="gap-4 has-[&gt;[data-slot=checkbox-group]]:gap-3 has-[&gt;[data-slot=radio-group]]:gap-3 flex flex-col"
    >
      <legend
        data-slot="field-legend"
        data-variant="legend"
        class="mb-1.5 font-medium data-[variant=label]:text-sm data-[variant=legend]:text-base"
      >
        Profile
      </legend>
      <p
        data-slot="field-description"
        class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
      >
        Fill in your profile information.
      </p>
      <div
        data-slot="field-group"
        class="gap-5 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col"
      >
        <div
          role="group"
          data-slot="field"
          data-orientation="responsive"
          class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full @md/field-group:flex-row @md/field-group:items-center @md/field-group:*:w-auto @md/field-group:has-[&gt;[data-slot=field-content]]:items-start @md/field-group:*:data-[slot=field-label]:flex-auto [&amp;&gt;.sr-only]:w-auto @md/field-group:has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
        >
          <div
            data-slot="field-content"
            class="gap-0.5 group/field-content flex flex-1 flex-col leading-snug"
          >
            <label
              data-slot="field-label"
              class="text-sm font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 items-center select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed has-data-checked:bg-primary/5 has-data-checked:border-primary/30 dark:has-data-checked:border-primary/20 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[&gt;[data-slot=field]]:rounded-lg has-[&gt;[data-slot=field]]:border has-[&gt;[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-muted/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:border-ring has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-ring/50 has-[&gt;[data-slot=field]]:has-[:focus-visible]:ring-3 *:data-[slot=field]:p-2.5 group/field-label peer/field-label flex w-fit has-[&gt;[data-slot=field]]:w-full has-[&gt;[data-slot=field]]:flex-col"
              for="name"
              >Name</label
            >
            <p
              data-slot="field-description"
              class="text-muted-foreground text-left text-sm [[data-variant=legend]+&amp;]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance last:mt-0 nth-last-2:-mt-1 [&amp;&gt;a]:underline [&amp;&gt;a]:underline-offset-4 [&amp;&gt;a:hover]:text-primary"
            >
              Provide your full name for identification
            </p>
          </div>
          <input
            data-slot="input"
            class="dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 disabled:bg-input/50 dark:disabled:bg-input/80 h-8 rounded-lg border bg-transparent px-2.5 py-1 text-base transition-colors file:h-6 file:text-sm file:font-medium focus-visible:ring-3 aria-invalid:ring-3 md:text-sm w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50"
            id="name"
            placeholder="Evil Rabbit"
            required=""
          />
        </div>
        <div
          role="group"
          data-slot="field"
          data-orientation="responsive"
          class="data-[invalid=true]:text-destructive gap-2 group/field flex w-full flex-col *:w-full @md/field-group:flex-row @md/field-group:items-center @md/field-group:*:w-auto @md/field-group:has-[&gt;[data-slot=field-content]]:items-start @md/field-group:*:data-[slot=field-label]:flex-auto [&amp;&gt;.sr-only]:w-auto @md/field-group:has-[&gt;[data-slot=field-content]]:[&amp;&gt;[role=checkbox],[role=radio]]:mt-px"
        >
          <button
            data-slot="button"
            data-variant="default"
            data-size="default"
            class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 bg-primary text-primary-foreground hover:bg-primary/80 h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
            type="submit"
          >
            Submit</button
          ><button
            data-slot="button"
            data-variant="outline"
            data-size="default"
            class="focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-lg border bg-clip-padding text-sm font-medium focus-visible:ring-3 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&amp;_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap transition-all outline-none select-none disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2"
            type="button"
          >
            Cancel
          </button>
        </div>
      </div>
    </fieldset>
  </form>
</div>
```
:::

::::


## Validation and Errors

- Add `data-invalid` to `Field` to switch the entire block into an error state.
- Add `aria-invalid` on the input itself for assistive technologies.
- Render `FieldError` immediately after the control or inside `FieldContent` to keep error messages aligned with the field.

```tsx showLineNumbers /data-invalid/ /aria-invalid/
<Field data-invalid>
  <FieldLabel htmlFor="email">Email</FieldLabel>
  <Input id="email" type="email" aria-invalid />
  <FieldError>Enter a valid email address.</FieldError>
</Field>
```

## Accessibility

- `FieldSet` and `FieldLegend` keep related controls grouped for keyboard and assistive tech users.
- `Field` outputs `role="group"` so nested controls inherit labeling from `FieldLabel` and `FieldLegend` when combined.
- Apply `FieldSeparator` sparingly to ensure screen readers encounter clear section boundaries.

## API Reference


**shadless surface** — every node is a `data-slot` attribute in the shipped markup.

| Slot |
| --- |
| `data-slot="field-set"` |
| `data-slot="field-legend"` |
| `data-slot="field-group"` |
| `data-slot="field"` |
| `data-slot="field-content"` |
| `data-slot="field-label"` |
| `data-slot="field-description"` |
| `data-slot="field-separator"` |
| `data-slot="field-separator-content"` |
| `data-slot="field-error"` |

**Runtime:** Each row below is a `cva`-declared variant baked into the shipped CSS as a `data-*` attribute; set it next to the slot's `data-slot` to pick that value (the Default needs no attribute). This table only covers `cva` variants — check `dist/css/field.css` for any other `data-*` selector on these slots.

| Slot | Attribute | Values | Default |
| --- | --- | --- | --- |
| `field` | `data-orientation` | `vertical`, `horizontal`, `responsive` | `vertical` |
See Installation → Files this component needs for the JavaScript this component requires.
