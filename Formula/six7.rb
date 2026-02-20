class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.23"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.23"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.23/six7-0.7.23-arm64-apple-darwin.tar.gz"
      sha256 "c8a197ee4bde94551936945aee28af7f05a444722c70da517635f8d92434f35a"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
