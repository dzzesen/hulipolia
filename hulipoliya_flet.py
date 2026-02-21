import flet as ft
from typing import cast

BASE_COLOR = ft.Colors.BLUE_GREY_400
HIGHLIGHT_COLOR = ft.Colors.BLUE_200

PAINT_COLORS = {
    "Red": ft.Colors.RED_400,
    "Blue": ft.Colors.BLUE_400,
    "Green": ft.Colors.GREEN_400,
    "Yellow": ft.Colors.YELLOW_400,
    "Purple": ft.Colors.PURPLE_400,
}

RIGHT_CONFIG = {
    "Gold": {"count": 16, "arrows": [4, 8, 11, 14, 16]},
    "Oil": {"count": 16, "arrows": [4, 8, 12, 16]},
    "Nasdaq": {"count": 15, "arrows": [3, 6, 9, 12, 15]},
    "Dow Jones": {"count": 14, "arrows": [4, 8, 11, 14]},
    "Bonds": {"count": 15, "arrows": [5, 10, 15]},
    "Country Stocks": {"count": 12, "arrows": [2, 5, 8, 12]},
}


def main(page: ft.Page):
    page.title = "Hulipoliya"
    page.bgcolor = ft.Colors.BLACK
    page.scroll = ft.ScrollMode.AUTO

    selected_color = {"value": ft.Colors.RED_400}

    wife_money = {"value": 20}
    husband_money = {"value": 20}
    wife_credit = {"value": 0}
    husband_credit = {"value": 0}

    def apply_money(change_field, money_state, money_text):
        try:
            delta = int(change_field.value)
            money_state["value"] += delta
            money_text.value = str(money_state["value"])
            change_field.value = ""
            page.update()
        except:
            pass

    def add_credit(money_state, money_text, credit_state, credit_text):
        money_state["value"] += 10
        credit_state["value"] += 1
        money_text.value = str(money_state["value"])
        credit_text.value = str(credit_state["value"])
        page.update()

    def subtract_credit(money_state, money_text, credit_state, credit_text):
        if money_state["value"] >= 10 and credit_state["value"] > 0:
            money_state["value"] -= 10
            credit_state["value"] -= 1
            money_text.value = str(money_state["value"])
            credit_text.value = str(credit_state["value"])
            page.update()

    palette = ft.Row(spacing=10)

    def select_color(color):
        selected_color["value"] = color
        for b in palette.controls:
            b = cast(ft.Container, b)
            b.border = ft.Border.all(
                3,
                ft.Colors.WHITE if b.bgcolor == color else ft.Colors.TRANSPARENT,
            )
        palette.update()

    for color in PAINT_COLORS.values():
        palette.controls.append(
            ft.Container(
                width=40,
                height=40,
                bgcolor=color,
                border_radius=6,
                border=ft.Border.all(3, ft.Colors.TRANSPARENT),
                on_click=lambda e, c=color: select_color(c),
            )
        )

    def paint(e):
        cell = cast(ft.Container, e.control)
        if cell.bgcolor == selected_color["value"]:
            cell.bgcolor = (
                HIGHLIGHT_COLOR if getattr(cell, "highlight", False) else BASE_COLOR
            )
        else:
            cell.bgcolor = selected_color["value"]
        cell.update()

    def make_left_scale(default_purple_cells=None):
        default_purple_cells = default_purple_cells or []
        cells = []
        for i in range(18):
            highlight = i in [0, 1, 2, 3, 16, 17]
            is_purple = i in default_purple_cells
            c = ft.Container(
                content=ft.Text(str(i), color="white"),
                width=32,
                height=32,
                alignment=ft.alignment.Alignment(0, 0),
                bgcolor=ft.Colors.PURPLE_400
                if is_purple
                else (HIGHLIGHT_COLOR if highlight else BASE_COLOR),
                border_radius=4,
                on_click=paint,
            )
            setattr(c, "highlight", highlight)
            cells.append(c)
        return ft.Row(cells, spacing=4)

    def make_arrow_rows(count, arrows):
        rows = []
        for row_idx in range(2):
            cells = []
            for i in range(1, count + 1):
                is_arrow = i in arrows
                margin = ft.Margin.only(right=10) if is_arrow else None
                arrow_char = "↘" if row_idx == 1 else "↗"
                cells.append(
                    ft.Container(
                        content=ft.Text(arrow_char if is_arrow else "", color="white"),
                        width=32,
                        height=32,
                        alignment=ft.alignment.Alignment(0, 0),
                        bgcolor=BASE_COLOR,
                        border_radius=4,
                        on_click=paint,
                        margin=margin,
                    )
                )
            rows.append(ft.Row(cells, spacing=4))
        return ft.Column(rows, spacing=4)

    rows = []

    DEFAULT_PRICES = {
        "Gold": [6, 7],
        "Oil": [7, 8],
        "Nasdaq": [8, 9],
        "Dow Jones": [8, 9],
        "Bonds": [9, 10],
        "Country Stocks": [5, 6],
    }

    for title, cfg in RIGHT_CONFIG.items():
        left = make_left_scale(DEFAULT_PRICES.get(title, []))
        arrows = make_arrow_rows(cfg["count"], cfg["arrows"])

        rows.append(
            ft.Row(
                [
                    left,
                    ft.Text(title, width=160, color="white", weight=ft.FontWeight.BOLD),
                    arrows,
                ],
                alignment=ft.MainAxisAlignment.START,
            )
        )

    wife_money_text = ft.Text("20", color="white", size=16)
    wife_credit_text = ft.Text("0", color="white", size=16)
    wife_change = ft.TextField(label="+ / - money", width=150)

    husband_money_text = ft.Text("20", color="white", size=16)
    husband_credit_text = ft.Text("0", color="white", size=16)
    husband_change = ft.TextField(label="+ / - money", width=150)

    page.add(
        ft.Column(
            [
                ft.Row(
                    [
                        ft.Column(
                            [
                                ft.Text(
                                    "Wife", color="white", weight=ft.FontWeight.BOLD
                                ),
                                ft.Row(
                                    [
                                        ft.Text("Credit:", color="white"),
                                        wife_credit_text,
                                        ft.Button(
                                            "-",
                                            on_click=lambda e: subtract_credit(
                                                wife_money,
                                                wife_money_text,
                                                wife_credit,
                                                wife_credit_text,
                                            ),
                                        ),
                                        ft.Button(
                                            "+",
                                            on_click=lambda e: add_credit(
                                                wife_money,
                                                wife_money_text,
                                                wife_credit,
                                                wife_credit_text,
                                            ),
                                        ),
                                    ]
                                ),
                                ft.Row(
                                    [ft.Text("Money:", color="white"), wife_money_text]
                                ),
                                ft.Row(
                                    [
                                        wife_change,
                                        ft.Button(
                                            "Apply",
                                            on_click=lambda e: apply_money(
                                                wife_change, wife_money, wife_money_text
                                            ),
                                        ),
                                    ]
                                ),
                            ]
                        ),
                        ft.Column(
                            [
                                ft.Text(
                                    "Husband", color="white", weight=ft.FontWeight.BOLD
                                ),
                                ft.Row(
                                    [
                                        ft.Text("Credit:", color="white"),
                                        husband_credit_text,
                                        ft.Button(
                                            "-",
                                            on_click=lambda e: subtract_credit(
                                                husband_money,
                                                husband_money_text,
                                                husband_credit,
                                                husband_credit_text,
                                            ),
                                        ),
                                        ft.Button(
                                            "+",
                                            on_click=lambda e: add_credit(
                                                husband_money,
                                                husband_money_text,
                                                husband_credit,
                                                husband_credit_text,
                                            ),
                                        ),
                                    ]
                                ),
                                ft.Row(
                                    [
                                        ft.Text("Money:", color="white"),
                                        husband_money_text,
                                    ]
                                ),
                                ft.Row(
                                    [
                                        husband_change,
                                        ft.Button(
                                            "Apply",
                                            on_click=lambda e: apply_money(
                                                husband_change,
                                                husband_money,
                                                husband_money_text,
                                            ),
                                        ),
                                    ]
                                ),
                            ]
                        ),
                    ],
                    spacing=40,
                ),
                ft.Divider(color="white"),
                ft.Text("Choose color:", color="white"),
                palette,
                ft.Divider(color="white"),
                *rows,
            ],
            spacing=14,
        )
    )

    select_color(selected_color["value"])


ft.run(main, view=ft.AppView.WEB_BROWSER)
