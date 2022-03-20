require_relative './work'

describe 'sign' do
    let!(:word_processor_role) {[
        { lag: 0, multiple: 1, denominator: 1 }
    ]}
    let!(:spreadsheet_role) {[
        { lag: 0, multiple: 2, denominator: 3 },
        { lag: 60*60*24*30, multiple: 1, denominator: 3 }
    ]}
    let!(:word_processor) { WordProcessor.new(word_processor_role) }
    let!(:spread_sheet) { Spreadsheet.new(spreadsheet_role) }
    let!(:ms_word) { Product.new('MS_WORD', word_processor, 18_800) }
    let!(:ichitaro) { Product.new('一太郎', word_processor, 20_000) }
    let!(:ms_excel) { Product.new('MS_EXCEL', spread_sheet, 27_800) }
    let!(:sanshiro) { Product.new('三四郎', spread_sheet, 5_000) }
    # let!(:sanshiro_two) { Product.new('三四郎2', spread_sheet, 3_002) }


    context 'signという関数は購入契約を取り扱い，売上の収益認識を行う．' do
        context '端数は可能な限り均等に，かつ早い日付から順に加える．' do
            context '購入契約がワードプロセッサの場合，購入日に全額1件の収益認識を行う．' do
                it '2022-03-08に「MS Word」を契約した場合，2022-03-08に18,800円の収益認識を行う．' do
                    sign_on = Time.new(2022, 3, 8)
                    actual = sign(ms_word, sign_on)
                    expect(actual).to eq [{ date: '2022-03-08', amount: 18_800 }]
                end

                it '2022-03-09に「MS Word」を契約した場合，2022-03-09に18,800円の収益認識を行う．' do
                    sign_on = Time.new(2022, 3, 9)
                    actual = sign(ms_word, sign_on)
                    expect(actual).to eq [{ date: '2022-03-09', amount: 18_800 }]
                end

                it '2022-03-08に「一太郎」を契約した場合，2022-03-08に20,000円の収益認識を行う．' do
                    sign_on = Time.new(2022, 3, 8)
                    actual = sign(ichitaro, sign_on)
                    expect(actual).to eq [{ date: '2022-03-08', amount: 20_000 }]
                end

                it '2022-03-09に「一太郎」を契約した場合，2022-03-09に20,000円の収益認識を行う．' do
                    sign_on = Time.new(2022, 3, 9)
                    actual = sign(ichitaro, sign_on)
                    expect(actual).to eq [{ date: '2022-03-09', amount: 20_000 }]
                end
            end

            context '購入契約がスプレッドシートの場合，契約日に売上の2/3、30日後に1/3を収益認識する．' do
                context '端数が1円となる製品「MS Excel」について' do
                    it '30日後が月をまたぐ2022-02-01に契約した場合，2022-02-01に 18,534円、2022-03-03に 9,266円収益認識を返す．' do
                        sign_on = Time.new(2022, 2, 1)
                        actual = sign(ms_excel, sign_on)
                        expected = [{ date: '2022-02-01', amount: 18_534 }, { date: '2022-03-03', amount: 9_266 }]
                        expect(actual).to eq expected
                    end
    
                    it '30日後も同月である2022-03-01に契約した場合，2022-03-01に 18,534円、2022-03-31に 9,266円収益認識を返す．' do
                        sign_on = Time.new(2022, 3, 1)
                        actual = sign(ms_excel, sign_on)
                        expected = [{ date: '2022-03-01', amount: 18_534 }, { date: '2022-03-31', amount: 9_266 }]
                        expect(actual).to eq expected
                    end
                end

                context '端数が1円となる製品「三四郎」について' do
                    it '30日後が月をまたぐ2022-02-01に契約した場合，2022-02-01に 3,334円、2022-03-03に 1,666円収益認識を返す' do
                        sign_on = Time.new(2022, 2, 1)
                        actual = sign(sanshiro, sign_on)
                        expected = [{ date: '2022-02-01', amount: 3_334 }, { date: '2022-03-03', amount: 1_666 }]
                        expect(actual).to eq expected
                    end
    
                    it '30日後も同月である2022-03-01に契約した場合，2022-03-01に 3,334円、2022-03-31に 1,666円収益認識を返す' do
                        sign_on = Time.new(2022, 3, 1)
                        actual = sign(sanshiro, sign_on)
                        expected = [{ date: '2022-03-01', amount: 3_334 }, { date: '2022-03-31', amount: 1_666 }]
                        expect(actual).to eq expected
                    end
                end

                # context '端数が2円となる製品「三四郎2」について' do
                #     it '30日後が月をまたぐ2022-02-01に契約した場合，2022-02-01に xxxx円、2022-03-03に xxxx円収益認識を返す'
    
                #     it '30日後も同月である2022-03-01に契約した場合，2022-03-01に xxxx円、2022-03-31に xxxx円収益認識を返す'
                # end
            end
        end
    end
end
